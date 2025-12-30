// Generated macro for impl_80 (impl)
macro_rules! Depcrate_expressionimpl_80 {
() => {
// Module: crate::expression
// Provides: {"impl_80"}
// Dependencies: {}
impl Serialize for Expression { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { match self { Self :: IntConstant (v) => serializer . serialize_i32 (* v) , Self :: FloatConstant (v) => serializer . serialize_f32 (* v) , Self :: BoolConstant (v) => serializer . serialize_bool (* v) , Self :: Identifier (..) => serializer . serialize_str (& self . to_string ()) , Self :: MacroCall (..) => serializer . serialize_str (& self . to_string ()) , _ => Expression :: serialize (self , serializer) , } } }
};
}
