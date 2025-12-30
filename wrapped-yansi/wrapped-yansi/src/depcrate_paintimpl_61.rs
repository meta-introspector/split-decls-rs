// Generated macro for impl_61 (impl)
macro_rules! Depcrate_paintimpl_61 {
() => {
// Module: crate::paint
// Provides: {"impl_61"}
// Dependencies: {}
impl < T > Painted < T > { # [doc = " Create a new [`Painted`] with a default [`Style`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use yansi::Painted;"] # [doc = ""] # [doc = " let painted = Painted::new(\"hello\");"] # [doc = " assert_eq!(painted.style, yansi::Style::new());"] # [doc = " ```"] # [inline (always)] pub const fn new (value : T) -> Painted < T > { Painted { value , style : Style :: new () } } # [inline (always)] const fn apply (mut self , a : crate :: style :: Application) -> Self { self . style = self . style . apply (a) ; self } # [inline] pub (crate) fn enabled (& self) -> bool { crate :: is_enabled () && self . style . condition . map_or (true , | c | c ()) } properties ! ([pub const] constructor (Self) -> Self) ; }
};
}
