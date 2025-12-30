// Generated macro for output_ty (function)
macro_rules! Depcrate_fn_utiloutput_ty {
() => {
// Module: crate::fn_util
// Provides: {"output_ty"}
// Dependencies: {}
pub fn output_ty (db_lt : Option < & syn :: Lifetime > , sig : & syn :: Signature) -> syn :: Result < syn :: Type > { match & sig . output { syn :: ReturnType :: Default => Ok (parse_quote ! (())) , syn :: ReturnType :: Type (_ , ty) => match db_lt { Some (db_lt) => Ok (ChangeLt :: elided_to (db_lt) . in_type (ty)) , None => Ok (syn :: Type :: clone (ty)) , } , } }
};
}
