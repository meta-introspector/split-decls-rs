// Generated macro for impl_extension (macro)
macro_rules! Depcrate_extimpl_extension {
() => {
// Module: crate::ext
// Provides: {"impl_extension"}
// Dependencies: {}
macro_rules ! impl_extension { ($ newtype : ty , critical = $ critical : expr) => { impl AsExtension for $ newtype { fn critical (& self , _subject : & Name , _extensions : & [Extension]) -> bool { $ critical } } } ; }
};
}
