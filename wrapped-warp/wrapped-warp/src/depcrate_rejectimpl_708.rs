// Generated macro for impl_708 (impl)
macro_rules! Depcrate_rejectimpl_708 {
() => {
// Module: crate::reject
// Provides: {"impl_708"}
// Dependencies: {}
impl Rejection { fn known (known : Known) -> Self { Rejection { reason : Reason :: Other (Box :: new (Rejections :: Known (known))) , } } fn custom (other : Box < dyn Cause >) -> Self { Rejection { reason : Reason :: Other (Box :: new (Rejections :: Custom (other))) , } } # [doc = " Searches this `Rejection` for a specific cause."] # [doc = ""] # [doc = " A `Rejection` will accumulate causes over a `Filter` chain. This method"] # [doc = " can search through them and return the first cause of this type."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " #[derive(Debug)]"] # [doc = " struct Nope;"] # [doc = ""] # [doc = " impl warp::reject::Reject for Nope {}"] # [doc = ""] # [doc = " let reject = warp::reject::custom(Nope);"] # [doc = ""] # [doc = " if let Some(nope) = reject.find::<Nope>() {"] # [doc = "    println!(\"found it: {:?}\", nope);"] # [doc = " }"] # [doc = " ```"] pub fn find < T : 'static > (& self) -> Option < & T > { if let Reason :: Other (ref rejections) = self . reason { return rejections . find () ; } None } # [doc = " Returns true if this Rejection was made via `warp::reject::not_found`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let rejection = warp::reject();"] # [doc = ""] # [doc = " assert!(rejection.is_not_found());"] # [doc = " ```"] pub fn is_not_found (& self) -> bool { matches ! (self . reason , Reason :: NotFound) } }
};
}
