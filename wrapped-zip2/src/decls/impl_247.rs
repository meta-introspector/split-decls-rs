macro_rules! deps {
    () => {
        FileOptions!();
        ExtendedFileOptions!();
        ZipResult!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl FileOptions < '_ , ExtendedFileOptions > { # [doc = " Adds an extra data field."] pub fn add_extra_data < D : AsRef < [u8] > > (& mut self , header_id : u16 , data : D , central_only : bool ,) -> ZipResult < () > { self . extended_options . add_extra_data (header_id , data , central_only) } # [doc = " Removes the extra data fields."] # [must_use] pub fn clear_extra_data (mut self) -> Self { if ! self . extended_options . extra_data . is_empty () { self . extended_options . extra_data = Arc :: new (vec ! []) ; } if ! self . extended_options . central_extra_data . is_empty () { self . extended_options . central_extra_data = Arc :: new (vec ! []) ; } self } }
    };
}

impl_247!()