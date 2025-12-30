// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < 's , 'a , T : 'a > Drop for VecMutScanItem < 's , 'a , T > { fn drop (& mut self) { unsafe { ptr :: copy (self . scan . base . add (self . scan . read) , self . scan . base . add (self . scan . write) , 1 ,) ; self . scan . read += 1 ; self . scan . write += 1 ; } } }
};
}
