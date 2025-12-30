// Generated macro for impl_105 (impl)
macro_rules! Depcrate_valueimpl_105 {
() => {
// Module: crate::value
// Provides: {"impl_105"}
// Dependencies: {}
# [cfg (feature = "display")] impl fmt :: Display for Value { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use serde_core :: Serialize as _ ; let mut output = String :: new () ; let serializer = crate :: ser :: ValueSerializer :: new (& mut output) ; self . serialize (serializer) . unwrap () ; output . fmt (f) } }
};
}
