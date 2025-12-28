macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! impl_1605 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a > Arg for Cow < 'a , OsStr > { # [inline] fn as_str (& self) -> io :: Result < & str > { (* * self) . to_str () . ok_or (io :: Errno :: INVAL) } # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { (* * self) . to_string_lossy () } # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { (& * * self) . into_c_str () } # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { match self { Cow :: Owned (os) => os . into_c_str () , Cow :: Borrowed (os) => os . into_c_str () , } } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { (& * self) . into_with_c_str (f) } }
    };
}

impl_1605!()