// Generated macro for impl_95 (impl)
macro_rules! Depcrate_future_maybe_doneimpl_95 {
() => {
// Module: crate::future::maybe_done
// Provides: {"impl_95"}
// Dependencies: {}
impl < Fut : Future > Future for MaybeDone < Fut > { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let output = match self . as_mut () . project () { MaybeDoneProj :: Future { future } => ready ! (future . poll (cx)) , MaybeDoneProj :: Done { .. } => return Poll :: Ready (()) , MaybeDoneProj :: Gone => panic ! ("MaybeDone polled after value taken") , } ; self . set (MaybeDone :: Done { output }) ; Poll :: Ready (()) } }
};
}
