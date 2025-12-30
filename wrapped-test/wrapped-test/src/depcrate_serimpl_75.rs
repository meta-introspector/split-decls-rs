// Generated macro for impl_75 (impl)
macro_rules! Depcrate_serimpl_75 {
() => {
// Module: crate::ser
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'a > Serializer < 'a > { # [doc = " Creates the serializer."] pub fn new (tokens : & 'a [Token]) -> Self { Serializer { tokens } } # [doc = " Pulls the next token off of the serializer, ignoring it."] fn next_token (& mut self) -> Option < Token > { if let Some ((& first , rest)) = self . tokens . split_first () { self . tokens = rest ; Some (first) } else { None } } pub fn remaining (& self) -> usize { self . tokens . len () } }
};
}
