macro_rules! deps {
    () => {
        ZipReadOptions!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < 'a > ZipReadOptions < 'a > { # [doc = " Create a new set of options with the default values."] # [must_use] pub fn new () -> Self { Self :: default () } # [doc = " Set the password, if any, to use.  Return for chaining."] # [must_use] pub fn password (mut self , password : Option < & 'a [u8] >) -> Self { self . password = password ; self } # [doc = " Set the ignore encryption flag.  Return for chaining."] # [must_use] pub fn ignore_encryption_flag (mut self , ignore : bool) -> Self { self . ignore_encryption_flag = ignore ; self } }
    };
}

impl_114!();