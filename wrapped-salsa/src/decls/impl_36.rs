macro_rules! deps {
    () => {
        Cancelled!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Cancelled { # [cold] pub (crate) fn throw (self) -> ! { panic :: resume_unwind (Box :: new (self)) ; } # [doc = " Runs `f`, and catches any salsa cancellation."] pub fn catch < F , T > (f : F) -> Result < T , Cancelled > where F : FnOnce () -> T + UnwindSafe , { match panic :: catch_unwind (f) { Ok (t) => Ok (t) , Err (payload) => match payload . downcast () { Ok (cancelled) => Err (* cancelled) , Err (payload) => panic :: resume_unwind (payload) , } , } } }
    };
}

impl_36!();