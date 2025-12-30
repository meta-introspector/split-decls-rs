// Generated macro for SerdeMapVisitor (struct)
macro_rules! DepcrateSerdeMapVisitor {
() => {
// Module: crate
// Provides: {"SerdeMapVisitor"}
// Dependencies: {}
# [doc = " Implements `tracing_core::field::Visit` for some `serde::ser::SerializeMap`."] # [derive (Debug)] pub struct SerdeMapVisitor < S : SerializeMap > { serializer : S , state : Result < () , S :: Error > , }
};
}
