use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: pagerank_rs_benchmark");
fn pagerank_rs_benchmark (c : & mut Criterion) { let mut group = c . benchmark_group ("pagerank_rs_group") ; group . sample_size (10) ; group . measurement_time (Duration :: from_secs (22)) ; let seed = 42 ; let mut rng = StdRng :: seed_from_u64 (seed) ; group . bench_function (BenchmarkId :: new ("pagerank_rs" , "") , | b | { let n = 100_000 ; let mut pagerank = Pagerank :: new (n) ; b . iter (| | { for from in 0 .. n { for _ in 0 .. rng . gen_range (0 .. 400) { let to = rng . gen_range (0 .. n) ; pagerank . link (black_box (from) , black_box (to)) . unwrap () ; } } pagerank . rank (black_box (0.85) , black_box (0.01)) ; pagerank . clear () ; }) ; }) ; group . finish () ; }
}