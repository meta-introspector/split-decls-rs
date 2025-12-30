// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl UnicodeXID for char { # [inline] fn is_xid_start (self) -> bool { ('a' <= self && self <= 'z') || ('A' <= self && self <= 'Z') || (self > '\x7f' && derived_property :: XID_Start (self)) } # [inline] fn is_xid_continue (self) -> bool { ('a' <= self && self <= 'z') || ('A' <= self && self <= 'Z') || ('0' <= self && self <= '9') || self == '_' || (self > '\x7f' && derived_property :: XID_Continue (self)) } }
};
}
