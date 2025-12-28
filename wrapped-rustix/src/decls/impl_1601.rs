macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! impl_1601 {
    () => {
        deps!();
        impl Arg for & CStr { # [inline] fn as_str (& self) -> io :: Result < & str > { self . to_str () . map_err (| _utf8_err | io :: Errno :: INVAL) } # [cfg (feature = "alloc")] # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { CStr :: to_string_lossy (self) } # [cfg (feature = "alloc")] # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { Ok (Cow :: Borrowed (self)) } # [cfg (feature = "alloc")] # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { Ok (Cow :: Borrowed (self)) } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { f (self) } }
    };
}

impl_1601!();