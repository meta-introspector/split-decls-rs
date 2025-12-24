use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Utility for writing benchmark tests.
///
/// A benchmark test looks like this:
///
/// ```ignore
/// #[test]
/// fn benchmark_foo() {
///     if skip_slow_tests() { return; }
///
///     let data = bench_fixture::some_fixture();
///     let analysis = some_setup();
///
///     let hash = {
///         let _b = bench("foo");
///         actual_work(analysis)
///     };
///     assert_eq!(hash, 92);
/// }
/// ```
///
/// * We skip benchmarks by default, to save time.
///   Ideal benchmark time is 800 -- 1500 ms in debug.
/// * We don't count preparation as part of the benchmark
/// * The benchmark itself returns some kind of numeric hash.
///   The hash is used as a sanity check that some code is actually run.
///   Otherwise, it's too easy to win the benchmark by just doing nothing.
pub fn bench(label: &'static str) -> impl Drop {
    struct Bencher {
        sw: StopWatch,
        label: &'static str,
    }
    impl Drop for Bencher {
        fn drop(&mut self) {
            eprintln!("{}: {}", self.label, self.sw.elapsed());
        }
    }
    Bencher {
        sw: StopWatch::start(),
        label,
    }
}
