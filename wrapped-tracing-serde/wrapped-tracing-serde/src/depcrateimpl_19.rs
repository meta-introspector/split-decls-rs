// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl Serialize for SerializeLevel < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if self . 0 == & Level :: ERROR { serializer . serialize_str ("ERROR") } else if self . 0 == & Level :: WARN { serializer . serialize_str ("WARN") } else if self . 0 == & Level :: INFO { serializer . serialize_str ("INFO") } else if self . 0 == & Level :: DEBUG { serializer . serialize_str ("DEBUG") } else if self . 0 == & Level :: TRACE { serializer . serialize_str ("TRACE") } else { unreachable ! () } } }
};
}
