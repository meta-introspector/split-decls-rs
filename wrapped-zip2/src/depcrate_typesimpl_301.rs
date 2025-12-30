// Generated macro for impl_301 (impl)
macro_rules! Depcrate_typesimpl_301 {
() => {
// Module: crate::types
// Provides: {"impl_301"}
// Dependencies: {}
impl TryFrom < (u16 , u16) > for DateTime { type Error = DateTimeRangeError ; # [inline] fn try_from (values : (u16 , u16)) -> Result < Self , Self :: Error > { Self :: try_from_msdos (values . 0 , values . 1) } }
};
}
