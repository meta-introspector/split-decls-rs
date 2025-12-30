// Generated macro for impl_127 (impl)
macro_rules! Depcrate_inline_tableimpl_127 {
() => {
// Module: crate::inline_table
// Provides: {"impl_127"}
// Dependencies: {}
impl < K : Into < Key > , V : Into < Value > > Extend < (K , V) > for InlineTable { fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { for (key , value) in iter { let key = key . into () ; let value = Item :: Value (value . into ()) ; self . items . insert (key , value) ; } } }
};
}
