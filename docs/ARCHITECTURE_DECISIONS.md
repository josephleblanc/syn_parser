# Current Architecture Decisions (as of 15-03-2025)

## ID System Strategy
- [ ] Consolidate to three core IDs: GraphNodeId (composite key), TypeId, TraitId
- [ ] Deprecate standalone NodeId (use GraphNodeId)
- [ ] Punted Decisions: \<link to specific docs\>

## Concurrency Approach
- Adopt gradual migration path outlined in `/docs/active/Concurrency_Migration/Visitor_Plan.md`
- First phase: Thread-safe visitor patterns before full async

