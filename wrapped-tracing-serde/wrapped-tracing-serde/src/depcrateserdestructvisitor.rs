// Generated macro for SerdeStructVisitor (struct)
macro_rules! DepcrateSerdeStructVisitor {
() => {
// Module: crate
// Provides: {"SerdeStructVisitor"}
// Dependencies: {}
# [doc = " Implements `tracing_core::field::Visit` for some `serde::ser::SerializeStruct`."] # [derive (Debug)] pub struct SerdeStructVisitor < S : SerializeStruct > { serializer : S , state : Result < () , S :: Error > , }
};
}
