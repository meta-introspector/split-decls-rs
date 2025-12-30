// Generated macro for impl_63 (impl)
macro_rules! Depcrate_kmacimpl_63 {
() => {
// Module: crate::kmac
// Provides: {"impl_63"}
// Dependencies: {}
impl IntoXof for Kmac { type Xof = KmacXof ; fn into_xof (mut self) -> Self :: Xof { self . state . update (right_encode (0) . value ()) ; KmacXof { state : self . state } } }
};
}
