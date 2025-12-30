// Generated macro for impl_73 (impl)
macro_rules! Depcrate_to_string_or_borrowimpl_73 {
() => {
// Module: crate::to_string_or_borrow
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'a > PartiallyValidatedUtf8 < 'a > { fn new (slice : & 'a [u8]) -> Self { Self { slice , offset : 0 } } # [doc = " Check whether the given string is the next chunk of unvalidated bytes."] # [doc = " If so, increment offset and return true. Otherwise, return false."] fn try_push (& mut self , valid_str : & str) -> bool { let new_offset = self . offset + valid_str . len () ; if self . slice . get (self . offset .. new_offset) == Some (valid_str . as_bytes ()) { self . offset = new_offset ; true } else { false } } # [doc = " Return the validated portion as `&str`."] fn validated_as_str (& self) -> & 'a str { debug_assert ! (self . offset <= self . slice . len ()) ; let valid_slice = unsafe { self . slice . get_unchecked (.. self . offset) } ; debug_assert ! (core :: str :: from_utf8 (valid_slice) . is_ok ()) ; unsafe { core :: str :: from_utf8_unchecked (valid_slice) } } }
};
}
