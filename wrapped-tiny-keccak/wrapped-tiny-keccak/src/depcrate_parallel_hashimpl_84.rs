// Generated macro for impl_84 (impl)
macro_rules! Depcrate_parallel_hashimpl_84 {
() => {
// Module: crate::parallel_hash
// Provides: {"impl_84"}
// Dependencies: {}
impl IntoXof for ParallelHash { type Xof = ParallelHashXof ; fn into_xof (mut self) -> Self :: Xof { if let Some (unfinished) = self . unfinished . take () { let mut suboutput = Suboutout :: security (self . bits) ; unfinished . state . finalize (suboutput . as_bytes_mut ()) ; self . state . update (suboutput . as_bytes ()) ; self . blocks += 1 ; } self . state . update (right_encode (self . blocks) . value ()) ; self . state . update (right_encode (0) . value ()) ; ParallelHashXof { state : self . state } } }
};
}
