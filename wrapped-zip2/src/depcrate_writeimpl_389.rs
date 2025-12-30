// Generated macro for impl_389 (impl)
macro_rules! Depcrate_writeimpl_389 {
() => {
// Module: crate::write
// Provides: {"impl_389"}
// Dependencies: {}
impl FileOptions < '_ , ExtendedFileOptions > { # [doc = " Adds an extra data field."] pub fn add_extra_data < D : AsRef < [u8] > > (& mut self , header_id : u16 , data : D , central_only : bool ,) -> ZipResult < () > { self . extended_options . add_extra_data (header_id , data , central_only) } # [doc = " Removes the extra data fields."] # [must_use] pub fn clear_extra_data (mut self) -> Self { if ! self . extended_options . extra_data . is_empty () { self . extended_options . extra_data = Arc :: new (vec ! []) ; } if ! self . extended_options . central_extra_data . is_empty () { self . extended_options . central_extra_data = Arc :: new (vec ! []) ; } self } }
};
}
