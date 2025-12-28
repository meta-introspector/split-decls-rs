macro_rules! deps {
    () => {
        RawString!();
        Repr!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl Repr { pub (crate) fn new_unchecked (raw : impl Into < RawString >) -> Self { Self { raw_value : raw . into () , } } # [doc = " Access the underlying value"] pub fn as_raw (& self) -> & RawString { & self . raw_value } # [doc = " The location within the original document"] # [doc = ""] # [doc = " This generally requires a [`Document`][crate::Document]."] pub fn span (& self) -> Option < std :: ops :: Range < usize > > { self . raw_value . span () } pub (crate) fn despan (& mut self , input : & str) { self . raw_value . despan (input) ; } # [cfg (feature = "display")] pub (crate) fn encode (& self , buf : & mut dyn std :: fmt :: Write , input : & str) -> std :: fmt :: Result { self . as_raw () . encode (buf , input) } }
    };
}

impl_219!()