macro_rules! deps {
    () => {
        ViewCaster!();
        ErasedDatabaseDownCasterSig!();
        DatabaseDownCasterSig!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl ViewCaster { fn new < DbView : ? Sized + Any > (func : DatabaseDownCasterSig < DbView >) -> ViewCaster { ViewCaster { target_type_id : TypeId :: of :: < DbView > () , type_name : std :: any :: type_name :: < DbView > () , cast : unsafe { mem :: transmute :: < DatabaseDownCasterSig < DbView > , ErasedDatabaseDownCasterSig > (func) } , } } }
    };
}

impl_424!();