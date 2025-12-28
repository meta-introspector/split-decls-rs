macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! impl_1603 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl Arg for CString { # [inline] fn as_str (& self) -> io :: Result < & str > { self . to_str () . map_err (| _utf8_err | io :: Errno :: INVAL) } # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { CStr :: to_string_lossy (self) } # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { Ok (Cow :: Borrowed (self)) } # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { Ok (Cow :: Owned (self)) } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { f (& self) } }
    };
}

impl_1603!();