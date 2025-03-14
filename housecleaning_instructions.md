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

#### A. AI Workflow Documents - Historical Tracking
| Document Path | Completed? | Category | Status | Rationale | Alternatives Considered |
|---------------|------------|----------|--------|-----------|-------------------------|
| `ai_next_task.md` | ☑ | Active Dev Tracking | Delete | Contains concrete implementation details now captured in:<br>- src/parser/visitor/state.rs:67-72 (ID generation)<br>- tests/common/mod.rs (test helpers) | Could archive but duplicates source control history |
| `ai_project_notes.md` | ☑ | Architectural Design | Archive | Valuable for:<br>1. Mermaid diagrams showing original visitor flow<br>2. Lesson learned about trait-impl disconnect<br>3. Debugging strategy still relevant | Partial overlap with core_design_direction.md section 3.2 |
| `ai_todo.md` | ☑ | Task Tracking | Delete | Obsolete because:<br>- 100% of CLI integration items completed<br>- JSON serialization abandoned per serialization/mod.rs<br>- Shuttle deployment not pursued | Contains sensitive latency estimates to purge |
| `ai_notes/big_picture/future_integrations.md` | ☐ | Speculative Design | Merge | Keep Qdrant integration plan (partially implemented)<br>Delete Rig.dev section (abandoned) | Needs redaction of deprecated approaches |
| `ai_notes/testing_status_start.md` | ☐ | QA History | Archive | Baseline metrics useful for:<br>- Tracking test coverage growth<br>- Benchmarking parser performance | Historical reference only - no active value |

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
