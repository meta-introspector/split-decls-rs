// Generated macro for impl_65 (impl)
macro_rules! Depcrate_expandimpl_65 {
() => {
// Module: crate::expand
// Provides: {"impl_65"}
// Dependencies: {}
impl VisitMut for ImplTraitEraser { fn visit_type_mut (& mut self , t : & mut Type) { if let Type :: ImplTrait (..) = t { * t = syn :: TypeInfer { underscore_token : Token ! [_] (t . span ()) , } . into () ; } else { syn :: visit_mut :: visit_type_mut (self , t) ; } } }
};
}
