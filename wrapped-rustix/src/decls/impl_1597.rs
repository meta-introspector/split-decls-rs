macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! impl_1597 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl Arg for OsString { # [inline] fn as_str (& self) -> io :: Result < & str > { self . as_os_str () . to_str () . ok_or (io :: Errno :: INVAL) } # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { self . as_os_str () . to_string_lossy () } # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { self . as_os_str () . into_c_str () } # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { # [cfg (all (target_os = "wasi" , target_env = "p2" , not (wasip2)))] return self . into_string () . map_err (| _strng_err | io :: Errno :: INVAL) ? . into_c_str () ; # [cfg (any (wasip2 , not (all (target_os = "wasi" , target_env = "p2"))))] self . into_vec () . into_c_str () } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { f (& self . into_c_str () ?) } }
    };
}

impl_1597!();