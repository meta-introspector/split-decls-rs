// Generated macro for impl_386 (impl)
macro_rules! Depcrate_writeimpl_386 {
() => {
// Module: crate::write
// Provides: {"impl_386"}
// Dependencies: {}
impl Debug for ExtendedFileOptions { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . write_fmt (format_args ! ("ExtendedFileOptions {{extra_data: vec!{:?}.into(), central_extra_data: vec!{:?}.into()}}" , self . extra_data , self . central_extra_data)) } }
};
}
