// Generated macro for impl_76 (impl)
macro_rules! Depcrate_to_string_or_borrowimpl_76 {
() => {
// Module: crate::to_string_or_borrow
// Provides: {"impl_76"}
// Dependencies: {}
impl < 'a > SliceOrString < 'a > { # [inline] fn new (slice : & 'a [u8]) -> Self { Self :: Slice (PartiallyValidatedUtf8 :: new (slice)) } # [inline] fn finish (self) -> Cow < 'a , str > { match self { SliceOrString :: Slice (slice) => Cow :: Borrowed (slice . validated_as_str ()) , SliceOrString :: String (owned) => Cow :: Owned (owned) , } } }
};
}
