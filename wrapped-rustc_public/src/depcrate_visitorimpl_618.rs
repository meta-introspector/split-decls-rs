// Generated macro for impl_618 (impl)
macro_rules! Depcrate_visitorimpl_618 {
() => {
// Module: crate::visitor
// Provides: {"impl_618"}
// Dependencies: {}
impl Visitable for Ty { fn visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { visitor . visit_ty (self) } fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self . kind () { super :: ty :: TyKind :: RigidTy (ty) => ty . visit (visitor) ? , super :: ty :: TyKind :: Alias (_ , alias) => alias . args . visit (visitor) ? , super :: ty :: TyKind :: Param (_) | super :: ty :: TyKind :: Bound (_ , _) => { } } ControlFlow :: Continue (()) } }
};
}
