// Generated macro for MaybeLiveLocals (struct)
macro_rules! Depcrate_impls_livenessMaybeLiveLocals {
() => {
// Module: crate::impls::liveness
// Provides: {"MaybeLiveLocals"}
// Dependencies: {}
# [doc = " A [live-variable dataflow analysis][liveness]."] # [doc = ""] # [doc = " This analysis considers references as being used only at the point of the"] # [doc = " borrow. In other words, this analysis does not track uses because of references that already"] # [doc = " exist. See [this `mir-dataflow` test][flow-test] for an example. You almost never want to use"] # [doc = " this analysis without also looking at the results of [`MaybeBorrowedLocals`]."] # [doc = ""] # [doc = " ## Field-(in)sensitivity"] # [doc = ""] # [doc = " As the name suggests, this analysis is field insensitive. If a projection of a variable `x` is"] # [doc = " assigned to (e.g. `x.0 = 42`), it does not \"define\" `x` as far as liveness is concerned. In fact,"] # [doc = " such an assignment is currently marked as a \"use\" of `x` in an attempt to be maximally"] # [doc = " conservative."] # [doc = ""] # [doc = " [`MaybeBorrowedLocals`]: super::MaybeBorrowedLocals"] # [doc = " [flow-test]: https://github.com/rust-lang/rust/blob/a08c47310c7d49cbdc5d7afb38408ba519967ecd/src/test/ui/mir-dataflow/liveness-ptr.rs"] # [doc = " [liveness]: https://en.wikipedia.org/wiki/Live_variable_analysis"] pub struct MaybeLiveLocals ;
};
}
