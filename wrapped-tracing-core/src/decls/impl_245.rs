macro_rules! deps {
    () => {
        Subscriber!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl dyn Subscriber + Send { # [doc = " Returns `true` if this [`Subscriber`] is the same type as `T`."] pub fn is < T : Any > (& self) -> bool { self . downcast_ref :: < T > () . is_some () } # [doc = " Returns some reference to this [`Subscriber`] value if it is of type `T`,"] # [doc = " or `None` if it isn't."] pub fn downcast_ref < T : Any > (& self) -> Option < & T > { unsafe { let raw = self . downcast_raw (TypeId :: of :: < T > ()) ? ; if raw . is_null () { None } else { Some (& * (raw as * const _)) } } } }
    };
}

impl_245!();