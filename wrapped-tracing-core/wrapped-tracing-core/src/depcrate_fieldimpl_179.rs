// Generated macro for impl_179 (impl)
macro_rules! Depcrate_fieldimpl_179 {
() => {
// Module: crate::field
// Provides: {"impl_179"}
// Dependencies: {}
impl < 'a , T : ? Sized > Value for & 'a mut T where T : Value + 'a , { fn record (& self , key : & Field , visitor : & mut dyn Visit) { T :: record (self , key , visitor) } }
};
}
