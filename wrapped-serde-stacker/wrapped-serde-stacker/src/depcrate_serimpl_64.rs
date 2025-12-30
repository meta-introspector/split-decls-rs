// Generated macro for impl_64 (impl)
macro_rules! Depcrate_serimpl_64 {
() => {
// Module: crate::ser
// Provides: {"impl_64"}
// Dependencies: {}
impl < T > ser :: Serialize for SerializeSized < T > where T : ser :: Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { stacker :: maybe_grow (self . param . red_zone , self . param . stack_size , | | { ser :: Serialize :: serialize (& self . value , Serializer { ser : serializer , red_zone : self . param . red_zone , stack_size : self . param . stack_size , } ,) }) } }
};
}
