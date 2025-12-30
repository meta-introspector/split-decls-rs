// Generated macro for impl_386 (impl)
macro_rules! Depcrate_thread_scopeimpl_386 {
() => {
// Module: crate::thread::scope
// Provides: {"impl_386"}
// Dependencies: {}
impl < F , T > Future for ScopeFuture < '_ , '_ , F , T > where F : Future < Output = T > , { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . 0 . as_mut () . project () { ScopeFutureProj :: Task { task , .. } => { let result = ready ! (task . poll (cx)) ; let ScopeFutureReplace :: Task { scope , .. } = this . 0 . as_mut () . project_replace (State :: None) else { unreachable ! ("found wrong state") } ; this . 0 . as_mut () . project_replace (State :: Wait { result , scope }) ; } ScopeFutureProj :: Wait { scope , .. } => { ready ! (scope . this . finish_async (cx)) ; let ScopeFutureReplace :: Wait { result , .. } = this . 0 . project_replace (State :: None) else { unreachable ! ("found wrong state") } ; return Poll :: Ready (result) ; } ScopeFutureProj :: None => panic ! ("`ScopeFuture` polled after completion") , } } } }
};
}
