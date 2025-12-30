// Generated macro for impl_62 (impl)
macro_rules! Depcrate_serimpl_62 {
() => {
// Module: crate::ser
// Provides: {"impl_62"}
// Dependencies: {}
impl core :: fmt :: Display for SerializerError { fn fmt (& self , formatter : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: InvalidFormat (e) => e . fmt (formatter) , Self :: InvalidProtocol => "invalid serialization protocol" . fmt (formatter) , } } }
};
}
