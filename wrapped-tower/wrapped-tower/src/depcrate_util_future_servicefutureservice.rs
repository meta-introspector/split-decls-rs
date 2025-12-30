// Generated macro for FutureService (struct)
macro_rules! Depcrate_util_future_serviceFutureService {
() => {
// Module: crate::util::future_service
// Provides: {"FutureService"}
// Dependencies: {}
# [doc = " A type that implements [`Service`] for a [`Future`] that produces a [`Service`]."] # [doc = ""] # [doc = " See [`future_service`] for more details."] # [derive (Clone)] pub struct FutureService < F , S > { state : State < F , S > , }
};
}
