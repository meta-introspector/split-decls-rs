// Generated macro for impl_147 (impl)
macro_rules! Depcrate_stateimpl_147 {
() => {
// Module: crate::state
// Provides: {"impl_147"}
// Dependencies: {}
impl < I > CircBuf < I > { pub fn append (& mut self , item : I) { self . idx = self . idx . checked_add (1) . and_then (| idx | idx . checked_rem (MAX_ITEMS)) . expect ("`self.idx` should be < `MAX_ITEMS` which should be non-zero") ; self . buf [self . idx] = item ; self . is_empty = false ; } pub fn buf (& self) -> & [I ; MAX_ITEMS] { & self . buf } pub fn last (& self) -> Option < & I > { if ! self . is_empty { self . buf . get (self . idx) } else { None } } }
};
}
