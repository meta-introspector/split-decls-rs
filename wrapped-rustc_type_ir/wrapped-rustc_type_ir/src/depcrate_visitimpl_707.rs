// Generated macro for impl_707 (impl)
macro_rules! Depcrate_visitimpl_707 {
() => {
// Module: crate::visit
// Provides: {"impl_707"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > , E : TypeVisitable < I > > TypeVisitable < I > for Result < T , E > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { match self { Ok (v) => v . visit_with (visitor) , Err (e) => e . visit_with (visitor) , } } }
};
}
