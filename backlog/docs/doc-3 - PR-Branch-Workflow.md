---
id: doc-3
title: PR-Branch-Workflow
status: published
created_date: '2026-05-24 16:00'
updated_date: '2026-05-24 16:00'
---

# PR-Branch-Workflow: Lokale Tooling sauber von Upstream-Commits trennen

## Problem

Wir haben lokale Dev-Artefakte (`.serena/`, `.claude/`, `backlog/`, `.gitignore`-Änderungen, `tmp/`), die **nicht** in Upstream-PRs (GNOME GitLab) landen dürfen. Aber wir wollen sie auf unserem lokalen `main` behalten.

## Workflow

Für jedes Feature:

```
    1. Branch starten
upstream main ──┬── feat/feature-x  ← nur Feature-Code
                │
    2. Merge in lokales main
lokal main  ────┴──────────────────  ← Feature + Tooling
```

### Schritt-für-Schritt

```bash
# 1. Branch erstellen (vom letzten CLEANEN upstream commit)
git checkout -b feat/feature-x <upstream-main-commit>

# 2. Entwickeln + Committen (NUR Feature-Code, kein Tooling!)
git add src/ data/  # nur source files
git commit -m "feat: ..."

# 3. Merge in lokales main (ALLES kommt mit)
git checkout main
git merge feat/feature-x    # main hat jetzt Feature + Tooling-Krams

# 4. Branch säubern für upstream PR
git checkout feat/feature-x
git rebase -i <upstream-main-commit>
  # → drop: tooling commits (.gitignore, .serena/, backlog/, .claude/)
  # → pick: feature commits
```

### Ausnahme: Gestapelte Branches (Stacked PRs)

Wenn Feature B auf Feature A aufbaut (z.B. `feat/playlist-detail` auf `feat/playlist-support`):

```bash
# Branch B vom Branch A starten (NICHT von main)
git checkout -b feat/feature-b feat/feature-a

# Nach Fertigstellung: erst A mergen, dann B rebasen
git checkout main
git merge feat/feature-a          # A ist in main
git checkout feat/feature-b
git rebase feat/feature-a          # B baut auf A auf

# Beide Branches säubern für PR
git rebase -i <upstream-main-commit>
  # → Beide Branches droppen tooling commits
```

### Wichtig

- **Niemals** Tooling-Commits in Feature-Branches committen (`.gitignore`, `backlog/`, `.serena/`, `.claude/`, `tmp/`)
- Vor `git add -A` IMMER `git status` checken — nicht versehentlich Tooling-Dateien erwischen
- Der Merge in lokales `main` ist der Moment wo Feature + Tooling zusammenkommen
- Nach dem Merge wird der Branch via Rebase gesäubert (nicht interaktiv — automatisiert mit `GIT_SEQUENCE_EDITOR`)
