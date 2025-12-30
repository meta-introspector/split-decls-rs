// Generated macro for ns_iter_inner (function)
macro_rules! Depcrate_benchns_iter_inner {
() => {
// Module: crate::bench
// Provides: {"ns_iter_inner"}
// Dependencies: {}
fn ns_iter_inner < T , F > (inner : & mut F , k : u64) -> u64 where F : FnMut () -> T , { let start = Instant :: now () ; for _ in 0 .. k { black_box (inner ()) ; } start . elapsed () . as_nanos () as u64 }
};
}
