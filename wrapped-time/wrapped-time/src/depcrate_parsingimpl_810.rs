// Generated macro for impl_810 (impl)
macro_rules! Depcrate_parsingimpl_810 {
() => {
// Module: crate::parsing
// Provides: {"impl_810"}
// Dependencies: {}
impl < 'a , T > ParsedItem < 'a , T > { # [doc = " Map the value to a new value, preserving the remaining input."] # [inline] pub (crate) fn map < U > (self , f : impl FnOnce (T) -> U) -> ParsedItem < 'a , U > { ParsedItem (self . 0 , f (self . 1)) } # [doc = " Map the value to a new, optional value, preserving the remaining input."] # [inline] pub (crate) fn flat_map < U > (self , f : impl FnOnce (T) -> Option < U >) -> Option < ParsedItem < 'a , U > > { Some (ParsedItem (self . 0 , f (self . 1) ?)) } # [doc = " Consume the stored value with the provided function. The remaining input is returned."] # [must_use = "this returns the remaining input"] # [inline] pub (crate) fn consume_value (self , f : impl FnOnce (T) -> Option < () >) -> Option < & 'a [u8] > { f (self . 1) ? ; Some (self . 0) } # [doc = " Filter the value with the provided function. If the function returns `false`, the value"] # [doc = " is discarded and `None` is returned. Otherwise, the value is preserved and `Some(self)` is"] # [doc = " returned."] # [inline] pub (crate) fn filter (self , f : impl FnOnce (& T) -> bool) -> Option < Self > { f (& self . 1) . then_some (self) } }
};
}
