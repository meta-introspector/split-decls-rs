// Generated macro for impl_23 (impl)
macro_rules! Depcrate_attr_quirkimpl_23 {
() => {
// Module: crate::attr_quirk
// Provides: {"impl_23"}
// Dependencies: {}
impl Attribute { pub (crate) fn fmt (& self , f : & mut dyn core :: fmt :: Write) -> core :: fmt :: Result { write ! (f , "{}" , match self { Attribute :: Bold => 1 , Attribute :: Dim => 2 , Attribute :: Italic => 3 , Attribute :: Underline => 4 , Attribute :: Blink => 5 , Attribute :: RapidBlink => 6 , Attribute :: Invert => 7 , Attribute :: Conceal => 8 , Attribute :: Strike => 9 , }) } # [doc = " Returns a `Style` with the attribute `self` enabled."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use yansi::{Style, Attribute::Bold};"] # [doc = ""] # [doc = " static EMBOLDEN: Style = Bold.style();"] # [doc = " ```"] pub const fn style (self) -> Style { Style :: new () . attr (self) } }
};
}
