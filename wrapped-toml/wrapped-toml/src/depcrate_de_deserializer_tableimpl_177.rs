// Generated macro for impl_177 (impl)
macro_rules! Depcrate_de_deserializer_tableimpl_177 {
() => {
// Module: crate::de::deserializer::table
// Provides: {"impl_177"}
// Dependencies: {}
impl < 'i > TableMapAccess < 'i > { pub (crate) fn new (input : TableDeserializer < 'i >) -> Self { Self { iter : input . items . into_iter () , span : input . span , value : None , } } }
};
}
