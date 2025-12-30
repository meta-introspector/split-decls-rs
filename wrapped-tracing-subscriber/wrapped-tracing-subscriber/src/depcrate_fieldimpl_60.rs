// Generated macro for impl_60 (impl)
macro_rules! Depcrate_fieldimpl_60 {
() => {
// Module: crate::field
// Provides: {"impl_60"}
// Dependencies: {}
impl < T , V , F > MakeVisitor < T > for F where F : Fn (T) -> V , V : Visit , { type Visitor = V ; fn make_visitor (& self , target : T) -> Self :: Visitor { (self) (target) } }
};
}
