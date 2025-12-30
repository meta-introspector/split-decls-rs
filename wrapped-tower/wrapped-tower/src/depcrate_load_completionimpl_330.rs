// Generated macro for impl_330 (impl)
macro_rules! Depcrate_load_completionimpl_330 {
() => {
// Module: crate::load::completion
// Provides: {"impl_330"}
// Dependencies: {}
impl < F , C , H , T , E > Future for TrackCompletionFuture < F , C , H > where F : Future < Output = Result < T , E > > , C : TrackCompletion < H , T > , { type Output = Result < C :: Output , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let rsp = ready ! (this . future . poll (cx)) ? ; let h = this . handle . take () . expect ("handle") ; Poll :: Ready (Ok (this . completion . track_completion (h , rsp))) } }
};
}
