use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn simple_pagerank_benchmark (c : & mut Criterion) { let mut group = c . benchmark_group ("simple_pagerank_group") ; group . sample_size (10) ; group . measurement_time (Duration :: from_secs (22)) ; let seed = 42 ; let mut rng = StdRng :: seed_from_u64 (seed) ; group . bench_function (BenchmarkId :: new ("simple_pagerank" , "") , | b | { b . iter (| | { let n = 100_000 ; let mut pr = SimplePagerank :: < usize > :: new () ; for from in 0 .. n { for _ in 0 .. rng . gen_range (0 .. 400) { let to = rng . gen_range (0 .. n) ; pr . add_edge (black_box (from) , black_box (to)) ; } } pr . nodes () . iter () . for_each (| (_node , _score) | { }) ; }) ; }) ; group . finish () ; }
}