// Generated macro for impl_61 (impl)
macro_rules! Depcrate_serimpl_61 {
() => {
// Module: crate::ser
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a , T > ser :: Serialize for Serialize < 'a , T > where T : ? Sized + ser :: Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { ser :: Serialize :: serialize (self . value , Serializer { ser : serializer , red_zone : self . param . red_zone , stack_size : self . param . stack_size , } ,) } }
};
}
