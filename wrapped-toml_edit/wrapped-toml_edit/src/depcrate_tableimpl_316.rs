// Generated macro for impl_316 (impl)
macro_rules! Depcrate_tableimpl_316 {
() => {
// Module: crate::table
// Provides: {"impl_316"}
// Dependencies: {}
impl < K : Into < Key > , V : Into < Item > > Extend < (K , V) > for Table { fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { for (key , value) in iter { let key = key . into () ; let value = value . into () ; self . items . insert (key , value) ; } } }
};
}
