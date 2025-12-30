// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'a , T : 'a > Drop for VecMutScan < 'a , T > { fn drop (& mut self) { unsafe { let suffix_len = self . end - self . read ; ptr :: copy (self . base . add (self . read) , self . base . add (self . write) , suffix_len ,) ; self . vec . set_len (self . write + suffix_len) ; } } }
};
}
