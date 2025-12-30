// Generated macro for impl_106 (impl)
macro_rules! Depcrate_valueimpl_106 {
() => {
// Module: crate::value
// Provides: {"impl_106"}
// Dependencies: {}
# [cfg (feature = "parse")] impl core :: str :: FromStr for Value { type Err = crate :: de :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { use serde_core :: Deserialize as _ ; Self :: deserialize (crate :: de :: ValueDeserializer :: parse (s) ?) } }
};
}
