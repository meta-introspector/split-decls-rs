// Generated macro for black_box (function)
macro_rules! Depcrate_benchblack_box {
() => {
// Module: crate::bench
// Provides: {"black_box"}
// Dependencies: {}
# [doc = " An identity function that *__hints__* to the compiler to be maximally pessimistic about what"] # [doc = " `black_box` could do."] # [doc = ""] # [doc = " See [`std::hint::black_box`] for details."] # [inline (always)] pub fn black_box < T > (dummy : T) -> T { std :: hint :: black_box (dummy) }
};
}
