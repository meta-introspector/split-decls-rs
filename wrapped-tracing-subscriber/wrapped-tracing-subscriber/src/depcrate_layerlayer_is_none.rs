// Generated macro for layer_is_none (function)
macro_rules! Depcrate_layerlayer_is_none {
() => {
// Module: crate::layer
// Provides: {"layer_is_none"}
// Dependencies: {}
# [doc = " Is a type implementing `Layer` `Option::<_>::None`?"] pub (crate) fn layer_is_none < L , S > (layer : & L) -> bool where L : Layer < S > , S : Subscriber , { unsafe { layer . downcast_raw (TypeId :: of :: < NoneLayerMarker > ()) } . is_some () }
};
}
