macro_rules! deps {
    () => {
        RawString!();
        Decor!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl Decor { # [doc = " Creates a new decor from the given prefix and suffix."] pub fn new (prefix : impl Into < RawString > , suffix : impl Into < RawString >) -> Self { Self { prefix : Some (prefix . into ()) , suffix : Some (suffix . into ()) , } } # [doc = " Go back to default decor"] pub fn clear (& mut self) { self . prefix = None ; self . suffix = None ; } # [doc = " Get the prefix."] pub fn prefix (& self) -> Option < & RawString > { self . prefix . as_ref () } # [cfg (feature = "display")] pub (crate) fn prefix_encode (& self , buf : & mut dyn std :: fmt :: Write , input : Option < & str > , default : & str ,) -> std :: fmt :: Result { if let Some (prefix) = self . prefix () { prefix . encode_with_default (buf , input , default) } else { write ! (buf , "{default}") } } # [doc = " Set the prefix."] pub fn set_prefix (& mut self , prefix : impl Into < RawString >) { self . prefix = Some (prefix . into ()) ; } # [doc = " Get the suffix."] pub fn suffix (& self) -> Option < & RawString > { self . suffix . as_ref () } # [cfg (feature = "display")] pub (crate) fn suffix_encode (& self , buf : & mut dyn std :: fmt :: Write , input : Option < & str > , default : & str ,) -> std :: fmt :: Result { if let Some (suffix) = self . suffix () { suffix . encode_with_default (buf , input , default) } else { write ! (buf , "{default}") } } # [doc = " Set the suffix."] pub fn set_suffix (& mut self , suffix : impl Into < RawString >) { self . suffix = Some (suffix . into ()) ; } pub (crate) fn despan (& mut self , input : & str) { if let Some (prefix) = & mut self . prefix { prefix . despan (input) ; } if let Some (suffix) = & mut self . suffix { suffix . despan (input) ; } } }
    };
}

impl_222!();