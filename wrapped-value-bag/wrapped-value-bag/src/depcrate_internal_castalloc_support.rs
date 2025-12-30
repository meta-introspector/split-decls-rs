// Generated macro for alloc_support (module)
macro_rules! Depcrate_internal_castalloc_support {
() => {
// Module: crate::internal::cast
// Provides: {"alloc_support"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_support { use super :: * ; use crate :: std :: borrow :: Cow ; impl < 'v > ValueBag < 'v > { # [doc = " Try get a `str` from this value."] # [doc = ""] # [doc = " This method is cheap for primitive types, but may call arbitrary"] # [doc = " serialization implementations for complex ones. If the serialization"] # [doc = " implementation produces a short lived string it will be allocated."] # [inline] pub fn to_str (& self) -> Option < Cow < 'v , str > > { self . inner . cast () . into_str () } } impl < 'v > Cast < 'v > { # [inline] pub (in crate :: internal) fn into_str (self) -> Option < Cow < 'v , str > > { match self { Cast :: Str (value) => Some (value . into ()) , Cast :: String (value) => Some (value . into ()) , _ => None , } } } # [cfg (test)] mod tests { # [cfg (target_arch = "wasm32")] use wasm_bindgen_test :: * ; use crate :: { std :: borrow :: ToOwned , test :: IntoValueBag , ValueBag } ; # [test] # [cfg_attr (target_arch = "wasm32" , wasm_bindgen_test)] fn primitive_cast () { let short_lived = "a string" . to_owned () ; assert_eq ! ("a string" , (&* short_lived) . into_value_bag () . to_borrowed_str () . expect ("invalid value")) ; assert_eq ! ("a string" , &* "a string" . into_value_bag () . to_str () . expect ("invalid value")) ; assert_eq ! ("a string" , (&* short_lived) . into_value_bag () . to_borrowed_str () . expect ("invalid value")) ; assert_eq ! ("a string" , ValueBag :: try_capture (& short_lived) . expect ("invalid value") . to_borrowed_str () . expect ("invalid value")) ; } } }
};
}
