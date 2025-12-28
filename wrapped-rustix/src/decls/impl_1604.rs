macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! impl_1604 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a > Arg for Cow < 'a , str > { # [inline] fn as_str (& self) -> io :: Result < & str > { Ok (self) } # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { Cow :: Borrowed (self) } # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { Ok (Cow :: Owned (CString :: new (self . as_ref ()) . map_err (| _cstr_err | io :: Errno :: INVAL) ? ,)) } # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { Ok (Cow :: Owned (match self { Cow :: Owned (s) => CString :: new (s) , Cow :: Borrowed (s) => CString :: new (s) , } . map_err (| _cstr_err | io :: Errno :: INVAL) ? ,)) } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { with_c_str (self . as_bytes () , f) } }
    };
}

impl_1604!();