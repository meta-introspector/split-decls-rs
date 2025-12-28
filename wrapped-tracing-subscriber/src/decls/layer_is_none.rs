macro_rules! deps {
    () => {
        Layer!();
        NoneLayerMarker!();
    };
}

macro_rules! layer_is_none {
    () => {
        deps!();
        # [doc = " Is a type implementing `Layer` `Option::<_>::None`?"] pub (crate) fn layer_is_none < L , S > (layer : & L) -> bool where L : Layer < S > , S : Subscriber , { unsafe { layer . downcast_raw (TypeId :: of :: < NoneLayerMarker > ()) } . is_some () }
    };
}

layer_is_none!()