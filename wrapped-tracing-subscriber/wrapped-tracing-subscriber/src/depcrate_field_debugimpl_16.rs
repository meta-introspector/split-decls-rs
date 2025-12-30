// Generated macro for impl_16 (impl)
macro_rules! Depcrate_field_debugimpl_16 {
() => {
// Module: crate::field::debug
// Provides: {"impl_16"}
// Dependencies: {}
impl < T , V > MakeVisitor < T > for Alt < V > where V : MakeVisitor < T > , { type Visitor = Alt < V :: Visitor > ; # [inline] fn make_visitor (& self , target : T) -> Self :: Visitor { Alt (self . 0 . make_visitor (target)) } }
};
}
