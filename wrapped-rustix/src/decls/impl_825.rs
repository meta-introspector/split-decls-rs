macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! impl_825 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a > Arg for Components < 'a > { # [inline] fn as_str (& self) -> io :: Result < & str > { self . as_path () . to_str () . ok_or (io :: Errno :: INVAL) } # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { self . as_path () . to_string_lossy () } # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { self . as_path () . into_c_str () } # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { self . as_path () . into_c_str () } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { self . as_path () . into_with_c_str (f) } }
    };
}

impl_825!()