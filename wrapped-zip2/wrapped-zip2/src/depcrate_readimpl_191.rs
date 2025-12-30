// Generated macro for impl_191 (impl)
macro_rules! Depcrate_readimpl_191 {
() => {
// Module: crate::read
// Provides: {"impl_191"}
// Dependencies: {}
impl < R : Read > Drop for ZipFile < '_ , R > { fn drop (& mut self) { if let Cow :: Owned (_) = self . data { if let Ok (mut inner) = self . take_raw_reader () { let _ = copy (& mut inner , & mut sink ()) ; } } } }
};
}
