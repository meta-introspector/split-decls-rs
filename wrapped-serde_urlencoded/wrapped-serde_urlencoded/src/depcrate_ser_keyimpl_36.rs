// Generated macro for impl_36 (impl)
macro_rules! Depcrate_ser_keyimpl_36 {
() => {
// Module: crate::ser::key
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'key > From < Key < 'key > > for Cow < 'static , str > { fn from (key : Key < 'key >) -> Self { match key { Key :: Static (key) => key . into () , Key :: Dynamic (key) => key . into_owned () . into () , } } }
};
}
