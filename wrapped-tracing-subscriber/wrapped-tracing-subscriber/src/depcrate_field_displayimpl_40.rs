// Generated macro for impl_40 (impl)
macro_rules! Depcrate_field_displayimpl_40 {
() => {
// Module: crate::field::display
// Provides: {"impl_40"}
// Dependencies: {}
impl < T , V > MakeVisitor < T > for Messages < V > where V : MakeVisitor < T > , { type Visitor = Messages < V :: Visitor > ; # [inline] fn make_visitor (& self , target : T) -> Self :: Visitor { Messages (self . 0 . make_visitor (target)) } }
};
}
