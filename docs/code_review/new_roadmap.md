First designed as of commit: refactor_type_system b9c89b3

---

### **Phase 1: Short-Term (0-2 Weeks) - Foundation & Immediate Wins**
**Objective**: Fix critical flaws while preserving workflow

1. **Semantic Type System Overhaul**
   ```rust
   // Before
   type_map: DashMap<String, TypeId> 

   // After
   type_map: DashMap<[u8; 16], TypeId> // Blake3 truncated hash
   fn type_fingerprint(ty: &Type) -> [u8; 16] {
       let full_hash = blake3::hash(ty.to_token_stream().as_bytes());
       full_hash.as_bytes()[..16].try_into().unwrap()
   }
   ```

2. **Concurrent ID Generation**
   ```rust
   struct IdPool {
       node_counter: AtomicUsize,
       type_counter: AtomicUsize,
   }
   ```

3. **State Decoupling**
   ```rust
   struct VisitorState {
       graph: GraphStore,
       types: TypeSystem,
       ids: IdPool,
       embeddings: EmbeddingCache,
   }
   ```

4. **RAG Preparation Pipeline**
   ```rust
   fn process_node(&self, node: Node) -> RagArtifact {
       RagArtifact {
           id: node.id,
           source_snippet: node.source_snippet(),
           metadata: node.metadata(),
           relations: node.relations()
       }
   }
   ```
   - Outputs structured data for external embedding service

**Success Metrics**:
- 40% reduction in type collisions
- 2.8x throughput increase on 16-core CPU
- 500k tokens/min processing baseline

---

### **Phase 2: Mid-Term (3-6 Weeks) - Hybrid Graph+Vector Core**
**Objective**: Implement core design's hybrid model

1. **CozoDB Integration**
   ```rust
   struct CozoGraphStore {
       tx: cozo::CozoTransaction,
       batch_size: AtomicUsize, // 9800X3D cache-aware
   }

   impl GraphStore for CozoGraphStore {
       fn add_node(&self, node: Node) {
           // Batch writes optimized for 32MB L3 cache
       }
   }
   ```

2. **Embedding Pipeline**
   ```rust
   enum Embedder {
       Cpu(all_MiniLM_L6_v2), //  RTX 3060 Ti fallback
       Cuda(CudaEmbedder), 
   }

   impl Embedder {
       async fn encode(&self, text: &str) -> Vec<f32> {
           match self {
               Self::Cpu(m) => m.encode(text), // 2k tok/s
               Self::Cuda(m) => m.encode(text).await, // 15k tok/s
           }
       }
   }
   ```

3. **RAG Context Builder**
   ```rust
   struct RagContext {
       graph: CozoGraphStore,
       vectors: HnswIndex, // 8GB VRAM optimized
   }

   impl RagContext {
       pub fn query(&self, prompt: &str) -> Vec<GraphNode> {
           let embedding = self.embedder.encode(prompt);
           self.graph.hybrid_search(embedding)
       }
   }
   ```

**Success Metrics**:
- 50ms hybrid query latency
- 8GB VRAM utilization for embeddings
- 90% cache hit rate on L3

---

### **Phase 3: Long-Term (7-12 Weeks) - Hardware-Optimized Production**
**Objective**: Full hardware-aware implementation

1. **GPU Acceleration**
   ```rust
   struct CudaEmbedder {
       model: Arc<Mutex<Bert>>, 
       stream: CudaStream,
       pinned_buffers: [DeviceBuffer<f32>; 2], // Double buffering
   }
   ```

2. **Incremental Parsing**
   ```rust
   struct VersionedGraph {
       current: Arc<Graph>,
       patches: Vec<GraphDelta>,
   }

   enum GraphDelta {
       Added(NodeId, Version),
       Updated(NodeId, Version, Patch),
       // ...
   }
   ```

3. **NUMA-Aware Processing**
   ```rust
   struct NumaAllocator {
       per_node_caches: [Cache; 4], // 4 CCDs on 9800X3D
   }

   impl NumaAllocator {
       fn alloc_for_thread(&self) -> &mut Cache {
           let cpu = current_cpu();
           &mut self.per_node_caches[cpu / 8] // 8 cores per CCD
       }
   }
   ```

4. **Distributed Processing**
   ```rust
   #[tonic::async_trait]
   impl ParserService for ParserNode {
       async fn parse_chunk(&self, req: ParseRequest) -> Result<ParseResponse> {
           // Load balance across NUMA nodes
       }
   }
   ```

**Success Metrics**:
- 95% GPU utilization (15k tok/s)
- 50μs incremental update latency
- Linear scaling to 4 nodes

---

### **Phase 4: Ongoing Optimization**

1. **Profile-Guided Optimization**
   ```bash
   cargo llvm-profile --target x86_64-unknown-linux-gnu \
     --branch-probabilities \
     --cache-line-size=64 # Zen4 specific
   ```

2. **Cache-Conscious Data Layouts**
   ```rust
   #[repr(C, align(64))] // Match L1d cache line
   struct NodeBundle {
       header: CacheLineHeader,
       data: [u8; 64 - mem::size_of::<CacheLineHeader>()],
   }
   ```

3. **Adaptive Batching**
   ```rust
   fn optimal_batch_size() -> usize {
       let l3 = cache_size::l3().unwrap_or(32_768_000);
       (l3 as f32 * 0.85) / ESTIMATED_NODE_SIZE
   }
   ```

---

### **Conclusion**

This plan incrementally transforms the visitor module into a hardware-optimized Rust parser that:

1. **Leverages Full Hardware Stack**:
   ```text
   RTX 3060 Ti → Embeddings
   9800X3D L3 → Type Cache
   DDR5 → Graph Storage
   ```

2. **Achieves Core Design Goals**:
   - 15ms hybrid queries via CozoDB+HNSW
   - 99% incremental update efficiency
   - 8GB VRAM / 32GB RAM utilization

3. **Enables Future Scaling**:
   ```rust
   struct DistributedParser {
       local: HardwareParser,
       cluster: ParserCluster,
       gpu_pool: CudaPool,
   }
   ```

By following this roadmap, you'll transform the current prototype into a production-grade system that fully utilizes your hardware while enabling real-time RAG workflows.
