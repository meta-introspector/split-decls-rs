// Generated macro for impl_1443 (impl)
macro_rules! Depcrate_serverimpl_1443 {
() => {
// Module: crate::server
// Provides: {"impl_1443"}
// Dependencies: {}
impl std :: future :: Future for WaitUntilZero { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> std :: task :: Poll < Self :: Output > { match self . info . upgrade () { None => std :: task :: Poll :: Ready (()) , Some (arc) => { let mut info = arc . lock () . expect ("we can't panic when holding lock") ; info . waker = Some (cx . waker () . clone ()) ; std :: task :: Poll :: Pending } } } }
};
}
