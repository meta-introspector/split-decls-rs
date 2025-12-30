// Generated macro for impl_72 (impl)
macro_rules! Depcrate_tuple_hashimpl_72 {
() => {
// Module: crate::tuple_hash
// Provides: {"impl_72"}
// Dependencies: {}
impl IntoXof for TupleHash { type Xof = TupleHashXof ; fn into_xof (mut self) -> TupleHashXof { self . state . update (right_encode (0) . value ()) ; TupleHashXof { state : self . state } } }
};
}
