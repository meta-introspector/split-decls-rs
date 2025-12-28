macro_rules! deps {
    () => {
        NoneLayerMarker!();
    };
}

macro_rules! subscriber_is_none {
    () => {
        deps!();
        # [doc = " Is a type implementing `Subscriber` `Option::<_>::None`?"] pub (crate) fn subscriber_is_none < S > (subscriber : & S) -> bool where S : Subscriber , { unsafe { subscriber . downcast_raw (TypeId :: of :: < NoneLayerMarker > ()) } . is_some () }
    };
}

subscriber_is_none!();