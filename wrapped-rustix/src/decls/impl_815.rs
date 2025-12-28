macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! impl_815 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl Arg for & Path { # [inline] fn as_str (& self) -> io :: Result < & str > { self . as_os_str () . to_str () . ok_or (io :: Errno :: INVAL) } # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { Path :: to_string_lossy (self) } # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { self . as_os_str () . into_c_str () } # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { self . as_os_str () . into_c_str () } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { self . as_os_str () . into_with_c_str (f) } }
    };
}

impl_815!()