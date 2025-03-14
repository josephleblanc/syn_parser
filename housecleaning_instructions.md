Let's create a structured plan to triage the documentation:

### Step 1: Establish Core Criteria
**Keep** documents that:
1. Contain eternal architectural decisions
2. Document patterns still in use
3. Explain non-obvious design choices
4. Track active development work

**Merge** documents that:
1. Cover similar/same subjects
2. Contain partial historical context
3. Have overlapping technical details

**Delete** documents that:
1. Describe abandoned approaches
2. Contain outdated implementation details
3. Duplicate info in core docs
4. Track completed/irrelevant work

### Step 2: Triage by Category

#### A. AI Workflow Documents
| Document | Recommendation | Reason |
|----------|----------------|--------|
| `ai_next_task.md` | Delete | Superseded by current implementation |
| `ai_project_notes.md` | Archive | Historical record of thought process |
| `ai_todo.md` | Delete | Contains outdated task lists |

#### B. ID System Refactoring
| Document | Recommendation | Reason |
|----------|----------------|--------|
| All `id_refactor/*.md` | Merge into single `history/id_refactor_arch.md` | Completed work, preserve context |

#### C. Performance/Concurrency
| Document | Recommendation | Reason |
|----------|----------------|--------|
| `concurrency_*` files | Keep condensed version | Still relevant to roadmap |
| `performance/*` | Merge with above | Related concerns |

#### D. Architecture/Design Docs
| Document | Recommendation | Reason |
|----------|----------------|--------|
| `Architectural_Revamp.md` | Merge into `core_design_direction.md` | Foundational changes |
| `visitor_architecture_approach.md` | Keep | Active patterns documentation |
| `different_approaches/*` | Archive | Historical alternatives |

#### E. Code Reviews
| Document | Recommendation | Reason |
|----------|----------------|--------|
| `code_review/*` | Merge into `design_decisions.md` | Preserve insights without clutter |

#### F. Testing Docs
| Document | Recommendation | Reason |
|----------|----------------|--------|
| `test_improvements.md` | Keep | Active testing strategy |
| `modular_tests.md` | Merge with above | Overlapping content |

#### G. Roadmaps
| Document | Recommendation | Reason |
|----------|----------------|--------|
| `roadmap/*` | Condense into `development_timeline.md` | Historical plans + current direction |

### Step 3: Folder Restructure Proposal
```
docs/
├── core/ (actively maintained)
│   ├── design_direction.md
│   ├── project_description.md
│   └── architecture.md (merged content)
├── development/
│   ├── active_work.md
│   └── timeline.md
├── adrs/ (architectural decision records)
├── testing/
│   └── strategy.md
└── archive/ (read-only historical)
    ├── ai_notes/
    └── id_refactor/
```

### Step 4: Execution Plan
1. **Week 1**: Archive AI-related documents
2. **Week 2**: Consolidate architecture/design docs
3. **Week 3**: Merge testing and roadmap docs
4. **Week 4**: Final review and delete deprecated files

### Final Recommendation
Keep the documentation focused on:
1. Current system truth (what exists now)
2. Decision records for key choices
3. Active development plans
4. Testing/QA processes

Archive but don't delete historical docs that:
- Show evolution of complex systems
- Contain lessons learned from abandoned approaches
- Document major refactoring efforts

Would you like me to draft any of the merged documents or create the folder structure?
