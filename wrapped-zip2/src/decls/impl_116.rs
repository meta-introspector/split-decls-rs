macro_rules! deps {
    () => {
        ZipFile!();
        ExtraField!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        # [doc = " Methods for retrieving information on zip files"] impl < R : Read > ZipFile < '_ , R > { # [doc = " iterate through all extra fields"] pub fn extra_data_fields (& self) -> impl Iterator < Item = & ExtraField > { self . data . extra_fields . iter () } }
    };
}

impl_116!()