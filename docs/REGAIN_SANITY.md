# Project Recovery Protocol

## 1. Emergency Git Operations

```bash
# Freeze current state in a backup branch
git checkout -b disaster-recovery-$(date +%s)

# Commit any residual changes
git add . && git commit -m "DISASTER STATE - Pre-recovery snapshot"

# List all branches to verify
git branch -av

# Return to main branch safely
git checkout main
```

## 2. Triage Priorities

- **Code Stability**: Rigid enforcement via `cargo check --all-targets`
- **Architecture**: Lock core systems (parser/graph modules)
- **AI Interventions**: Quarantine experimental changes

## 3. Collaborative Workflow Enforcement

```rust
#[derive(Protocol)]
enum AIAction {
    // All AI-proposed changes require:
    ImpactAssessment(String), // Affected modules
    RollbackPlan(String),     // Reversion steps
    DocumentationTriage(bool)// ADR required
}

impl Validation {
    fn ai_commit_policy(&self) -> Vec<Check> {
        vec![
            Check::CyclicDependencies, // Absolute ban
            Check::CoreModification,   // Requires human + AI co-sign
            Check::ArchitectureDrift   // Measured against last stable
        ]
    }
}
```

## 4. Documentation Recovery Matrix

```python
def salvage_docs(root_dir: str) -> Iterator[str]:
    '''Mine valuable fragments using:
    - Architectural keywords
    - Decision rationale markers
    - Post-mortem insights'''
    patterns = [
        r'ARCHITECTURAL\s+DECISION',
        r'CORE\s+INVARIANT',
        r'LESSONS\s+LEARNED'
    ]
    for path in Path(root_dir).rglob('*.md'):
        with open(path) as f:
            if any(re.search(p, f.read(), re.I) for p in patterns):
                yield str(path)
```

## 5. Continuous Vigilance System

```bash
# Install guardian angel hook
ln -s ../../scripts/pre-commit.guardian .git/hooks/pre-commit

# Sample hook script (scripts/pre-commit.guardian):
#!/bin/sh
cargo check || { echo "COMMIT REJECTED: Compilation broken"; exit 1; }
git diff --cached --name-only | xargs grep -iE 'AI_INTERVENTION' && \
  [ -f ADR/ai_change.md ] || { echo "Missing ADR for AI changes"; exit 1; }
```
