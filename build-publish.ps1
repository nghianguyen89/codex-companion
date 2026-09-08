$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot
foreach ($tool in @('node', 'cargo')) {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) { throw "Install $tool before building; see BUILD.md." }
}
if (-not (Test-Path -LiteralPath 'node_modules/@tauri-apps/cli/tauri.js')) {
    & pnpm install --config.confirmModulesPurge=false
    if ($LASTEXITCODE -ne 0) { throw 'Dependency installation failed.' }
}
& node node_modules/@tauri-apps/cli/tauri.js build --bundles nsis
if ($LASTEXITCODE -ne 0) { throw 'Native build failed.' }
$portable = Join-Path $PSScriptRoot 'release/portable'
New-Item -ItemType Directory -Force -Path $portable | Out-Null
Copy-Item -LiteralPath 'src-tauri/target/release/dev-companion.exe' -Destination $portable
Set-Content -LiteralPath (Join-Path $portable 'portable-mode') -Value ''
Copy-Item -LiteralPath 'docs/USER_GUIDE.md' -Destination $portable
Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $portable 'dev-companion.exe') | Format-List
