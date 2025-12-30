// Generated macro for impl_457 (impl)
macro_rules! Depcrate_ser_document_strategyimpl_457 {
() => {
// Module: crate::ser::document::strategy
// Provides: {"impl_457"}
// Dependencies: {}
impl < T > From < & T > for SerializationStrategy where T : serde_core :: ser :: Serialize + ? Sized , { fn from (value : & T) -> Self { value . serialize (WalkValue) . unwrap_err () } }
};
}
