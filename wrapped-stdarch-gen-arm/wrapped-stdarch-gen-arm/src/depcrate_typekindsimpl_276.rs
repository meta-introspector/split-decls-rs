// Generated macro for impl_276 (impl)
macro_rules! Depcrate_typekindsimpl_276 {
() => {
// Module: crate::typekinds
// Provides: {"impl_276"}
// Dependencies: {}
impl ToRepr for TypeKind { fn repr (& self , repr : TypeRepr) -> String { match self { Self :: Vector (ty) => ty . repr (repr) , Self :: Pointer (ty , _) => ty . repr (repr) , Self :: Base (ty) => ty . repr (repr) , Self :: Wildcard (w) => format ! ("{w}") , Self :: Custom (s) => s . to_string () , } } }
};
}
