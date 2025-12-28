macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! impl_812 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl Arg for & OsStr { # [inline] fn as_str (& self) -> io :: Result < & str > { self . to_str () . ok_or (io :: Errno :: INVAL) } # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { OsStr :: to_string_lossy (self) } # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { self . into_c_str () } # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { # [cfg (all (target_os = "wasi" , target_env = "p2" , not (wasip2)))] return self . to_str () . ok_or (io :: Errno :: INVAL) ? . into_c_str () ; # [cfg (any (wasip2 , not (all (target_os = "wasi" , target_env = "p2"))))] return self . as_bytes () . into_c_str () ; } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { # [cfg (all (target_os = "wasi" , target_env = "p2" , not (wasip2)))] return self . as_str () ? . into_with_c_str (f) ; # [cfg (any (wasip2 , not (all (target_os = "wasi" , target_env = "p2"))))] return self . as_bytes () . into_with_c_str (f) ; } }
    };
}

impl_812!()