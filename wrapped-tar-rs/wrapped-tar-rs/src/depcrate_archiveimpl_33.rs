// Generated macro for impl_33 (impl)
macro_rules! Depcrate_archiveimpl_33 {
() => {
// Module: crate::archive
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a , R : Read > Entries < 'a , R > { # [doc = " Indicates whether this iterator will return raw entries or not."] # [doc = ""] # [doc = " If the raw list of entries is returned, then no preprocessing happens"] # [doc = " on account of this library, for example taking into account GNU long name"] # [doc = " or long link archive members. Raw iteration is disabled by default."] pub fn raw (self , raw : bool) -> Entries < 'a , R > { Entries { fields : EntriesFields { raw , .. self . fields } , _ignored : marker :: PhantomData , } } }
};
}
