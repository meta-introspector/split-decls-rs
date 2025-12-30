// Generated macro for SubscriberExt (trait)
macro_rules! Depcrate_layerSubscriberExt {
() => {
// Module: crate::layer
// Provides: {"SubscriberExt"}
// Dependencies: {}
# [doc = " Extension trait adding a `with(Layer)` combinator to `Subscriber`s."] pub trait SubscriberExt : Subscriber + crate :: sealed :: Sealed { # [doc = " Wraps `self` with the provided `layer`."] fn with < L > (self , layer : L) -> Layered < L , Self > where L : Layer < Self > , Self : Sized , { layer . with_subscriber (self) } }
};
}
