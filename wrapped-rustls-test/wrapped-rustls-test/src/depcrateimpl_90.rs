// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl ClientStorage { pub fn new () -> Self { Self { storage : Arc :: new (rustls :: client :: ClientSessionMemoryCache :: new (1024)) , ops : Mutex :: new (Vec :: new ()) , alter_max_early_data_size : None , } } pub fn alter_max_early_data_size (& mut self , expected : u32 , altered : u32) { self . alter_max_early_data_size = Some ((expected , altered)) ; } pub fn ops (& self) -> Vec < ClientStorageOp > { self . ops . lock () . unwrap () . clone () } pub fn ops_and_reset (& self) -> Vec < ClientStorageOp > { mem :: take (& mut self . ops . lock () . unwrap ()) } }
};
}
