// Generated macro for impl_19 (impl)
macro_rules! Depcrate_enumerableimpl_19 {
() => {
// Module: crate::enumerable
// Provides: {"impl_19"}
// Dependencies: {}
impl < T , E > Valuable for Result < T , E > where T : Valuable , E : Valuable , { fn as_value (& self) -> Value < '_ > { Value :: Enumerable (self) } fn visit (& self , visitor : & mut dyn Visit) { match self { Ok (val) => visitor . visit_unnamed_fields (& [val . as_value ()]) , Err (val) => visitor . visit_unnamed_fields (& [val . as_value ()]) , } } }
};
}
