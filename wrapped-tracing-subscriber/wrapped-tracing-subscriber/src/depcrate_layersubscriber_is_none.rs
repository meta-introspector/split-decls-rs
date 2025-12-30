// Generated macro for subscriber_is_none (function)
macro_rules! Depcrate_layersubscriber_is_none {
() => {
// Module: crate::layer
// Provides: {"subscriber_is_none"}
// Dependencies: {}
# [doc = " Is a type implementing `Subscriber` `Option::<_>::None`?"] pub (crate) fn subscriber_is_none < S > (subscriber : & S) -> bool where S : Subscriber , { unsafe { subscriber . downcast_raw (TypeId :: of :: < NoneLayerMarker > ()) } . is_some () }
};
}
