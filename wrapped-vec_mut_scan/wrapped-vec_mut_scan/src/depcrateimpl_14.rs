// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'a , T : 'a > Drop for VecGrowScan < 'a , T > { fn drop (& mut self) { if self . queue . is_empty () { unsafe { let suffix_len = self . end - self . read ; ptr :: copy (self . base . add (self . read) , self . base . add (self . write) , suffix_len ,) ; self . vec . set_len (self . write + suffix_len) ; } } else { unsafe { self . vec . set_len (self . end) ; } self . vec . splice (self . write .. self . write , mem :: replace (& mut self . queue , VecDeque :: new ()) . into_iter () ,) ; } } }
};
}
