// Generated macro for impl_616 (impl)
macro_rules! Depcrate_steerimpl_616 {
() => {
// Module: crate::steer
// Provides: {"impl_616"}
// Dependencies: {}
impl < S , F , Req > Steer < S , F , Req > { # [doc = " Make a new [`Steer`] with a list of [`Service`]'s and a [`Picker`]."] # [doc = ""] # [doc = " Note: the order of the [`Service`]'s is significant for [`Picker::pick`]'s return value."] pub fn new (services : impl IntoIterator < Item = S > , router : F) -> Self { let services : Vec < _ > = services . into_iter () . collect () ; let not_ready : VecDeque < _ > = services . iter () . enumerate () . map (| (i , _) | i) . collect () ; Self { router , services , not_ready , _phantom : PhantomData , } } }
};
}
