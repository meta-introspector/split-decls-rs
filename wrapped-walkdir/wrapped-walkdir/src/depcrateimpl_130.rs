// Generated macro for impl_130 (impl)
macro_rules! Depcrateimpl_130 {
() => {
// Module: crate
// Provides: {"impl_130"}
// Dependencies: {}
impl DirList { fn close (& mut self) { if let DirList :: Opened { .. } = * self { * self = DirList :: Closed (self . collect :: < Vec < _ > > () . into_iter ()) ; } } }
};
}
