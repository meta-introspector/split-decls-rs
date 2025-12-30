// Generated macro for impl_15 (impl)
macro_rules! Depcrate_arrayimpl_15 {
() => {
// Module: crate::array
// Provides: {"impl_15"}
// Dependencies: {}
impl < V : Into < Value > > Extend < V > for Array { fn extend < T : IntoIterator < Item = V > > (& mut self , iter : T) { for value in iter { self . push_formatted (value . into ()) ; } } }
};
}
