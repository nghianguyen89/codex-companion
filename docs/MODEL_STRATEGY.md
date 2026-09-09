# Codex Model Strategy

## Goal

Use the lowest-cost model that can reliably complete the task.

Choose based on:

```text
Complexity
Uncertainty
Risk
```

Do not choose based only on task size.

---

## Luna — Explore & Routine

Use for:

- repository exploration
- locating files, symbols, references, dependencies
- reading and summarizing code
- straightforward renames
- repetitive edits following a clear pattern
- simple UI/CSS work
- small configuration/documentation changes
- low-risk fixes with an obvious solution

Avoid Luna when:

- architecture decisions are required
- root cause is unclear
- several systems interact in complex ways
- regression risk is significant

---

## Terra — Implement

Default implementation model.

Use for:

- normal features
- bug fixes with reasonably clear scope
- refactoring
- unit/integration tests
- API integration
- component/module implementation
- approved implementation plans
- medium multi-file changes
- mechanical code transformations requiring moderate reasoning

---

## Sol — Plan, Reason, Orchestrate & Review

Use for:

- implementation planning
- architecture analysis
- multi-agent orchestration
- complex debugging
- root-cause investigation
- cross-module reasoning
- important code reviews
- new module design
- comparing technical approaches
- performance/concurrency issues
- security-sensitive work
- architecture-affecting refactors

Typical flow:

```text
Sol   -> Plan/orchestrate
Terra -> Implement
Sol   -> Review when needed
```

---

## Astra — Escalation

Reserve for exceptional complexity.

Use for:

- major architecture changes with significant uncertainty
- large migrations with unclear consequences
- complex system-wide debugging
- deep multi-layer reasoning
- critical production issues
- high-risk refactors
- problems Sol cannot resolve confidently
- broad investigations where an incorrect decision is costly

Do not use Astra as the default implementation model.

---

# Quick Routing

| Task | Preferred model |
|---|---|
| Find usages / search repo | Luna |
| Read and summarize modules | Luna |
| Straightforward rename | Luna |
| Repetitive edit | Luna |
| Simple UI/CSS fix | Luna |
| Normal feature | Terra |
| Unit/integration tests | Terra |
| Existing-module refactor | Terra |
| Implement approved plan | Terra |
| Investigate unclear bug | Sol |
| Design new module | Sol |
| Architecture review | Sol |
| Plan large feature | Sol |
| Orchestrate multi-agent task | Sol |
| Complex regression | Sol |
| Major uncertain migration | Sol / Astra |
| System-wide redesign | Astra |
| Extremely difficult unresolved bug | Astra |

---

# Cost Control

- Avoid expensive models for repetitive work.
- Separate planning from implementation for complex work.
- Reuse `CURRENT_STATE.md`, `ARCHITECTURE.md`, and existing plans.
- Keep exploration focused after the affected area is known.
- Preserve stable conclusions so future sessions do not rediscover them.
- Do not create subagents for work the main agent can cheaply finish directly.
