// Generated macro for impl_29 (impl)
macro_rules! Depcrate_hypervisor_vspaceimpl_29 {
() => {
// Module: crate::hypervisor::vspace
// Provides: {"impl_29"}
// Dependencies: {}
impl fmt :: Display for MapAction { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use self :: MapAction :: * ; match self { MapAction :: None => write ! (f , " ---") , ReadUser => write ! (f , "uR--") , ReadKernel => write ! (f , "kR--") , ReadWriteUser => write ! (f , "uRW-") , ReadWriteKernel => write ! (f , "kRW-") , ReadExecuteUser => write ! (f , "uR-X") , ReadExecuteKernel => write ! (f , "kR-X") , ReadWriteExecuteUser => write ! (f , "uRWX") , ReadWriteExecuteKernel => write ! (f , "kRWX") , } } }
};
}
