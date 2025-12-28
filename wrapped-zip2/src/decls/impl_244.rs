macro_rules! deps {
    () => {
        ExtendedFileOptions!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl Debug for ExtendedFileOptions { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . write_fmt (format_args ! ("ExtendedFileOptions {{extra_data: vec!{:?}.into(), central_extra_data: vec!{:?}.into()}}" , self . extra_data , self . central_extra_data)) } }
    };
}

impl_244!();