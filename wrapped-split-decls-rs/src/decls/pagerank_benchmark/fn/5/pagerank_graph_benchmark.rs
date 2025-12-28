use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn pagerank_graph_benchmark (c : & mut Criterion) { let mut group = c . benchmark_group ("pagerank_graph_group") ; group . sample_size (10) ; group . measurement_time (Duration :: from_secs (22)) ; let seed = 42 ; let mut rng = StdRng :: seed_from_u64 (seed) ; group . bench_function (BenchmarkId :: new ("pagerank_graph" , "") , | b | { b . iter (| | { let n = 100_000 ; let mut edges = Vec :: new () ; for from in 0 .. n { for _ in 0 .. rng . gen_range (0 .. 400) { let to = rng . gen_range (0 .. n) ; edges . push ((from , to)) ; } } let graph : DirectedCsrGraph < usize > = GraphBuilder :: new () . edges (edges) . build () ; let (_ranks , _ , _) = page_rank (& graph , PageRankConfig :: new (10 , 1E-4 , black_box (0.85))) ; }) ; }) ; group . finish () ; }
}