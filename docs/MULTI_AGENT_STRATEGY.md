# Multi-Agent Strategy

## Purpose

Use multi-agent delegation to reduce context/cost, parallelize independent work,
and improve quality on complex tasks.

Multi-agent is a tool, not a mandatory ceremony.

## Preferred Roles

```text
Explorer  -> Luna
Architect -> Sol
Worker    -> Terra
Reviewer  -> Sol
```

Astra remains an escalation option rather than a routine child role.

## Default Complex Flow

```text
Main orchestrator / Sol
        │
        ├── Explorer / Luna
        │     repository discovery, search, bounded analysis
        │
        ├── Architect / Sol
        │     planning and architecture-sensitive reasoning
        │
        ├── Worker / Terra
        │     implementation, refactoring, tests
        │
        └── Reviewer / Sol
              review, regressions, architectural consistency
```

This is a routing pattern, not a requirement to instantiate all four roles.

## Delegation Rules

Delegate when at least one is true:

- the child task is independently bounded;
- a cheaper model can perform it reliably;
- parallel work materially saves time;
- isolating context improves focus;
- an independent review materially improves quality.

Do not delegate when:

- the task is trivial;
- delegation overhead exceeds the work;
- the child would need nearly the entire parent conversation;
- two agents would edit the same code concurrently without coordination.

## Model Selection

When the collaboration tool allows explicit child model selection:

- choose Luna for exploration/routine work;
- choose Terra for implementation;
- choose Sol for architecture/reasoning/review;
- choose Astra only when exceptional complexity requires it.

If the tool does not permit explicit model selection, keep the role/task boundary
and use the best available supported behavior rather than inventing configuration.

## Context Discipline

Give each child a concise task brief containing only what it needs:

```text
Goal
Relevant files/modules
Constraints
Expected output
Validation expectations
```

When the tooling supports fresh or bounded child context, prefer it over copying
the entire parent history unless full history is genuinely required.

## Parallelism

Good candidates for parallel work:

- independent repository searches
- independent test investigation
- separate risk/review passes
- unrelated modules with no overlapping edits

Avoid parallel code edits to the same files unless coordination is explicit.

## Handoff

A child should return a concise result:

```text
Findings
Files affected
Decisions / assumptions
Risks
Recommended next step
```

The main orchestrator remains responsible for the final integrated result.

## Escalation

Escalate only when needed:

```text
Luna -> Terra -> Sol -> Astra
```

A failed attempt alone is not proof that a stronger model is needed. Check for
missing context, bad assumptions, environment errors, and unclear requirements first.
