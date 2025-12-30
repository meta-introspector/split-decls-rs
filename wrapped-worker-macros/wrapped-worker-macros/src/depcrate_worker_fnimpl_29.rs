// Generated macro for impl_29 (impl)
macro_rules! Depcrate_worker_fnimpl_29 {
() => {
// Module: crate::worker_fn
// Provides: {"impl_29"}
// Dependencies: {}
impl < F > Parse for WorkerFn < F > where F : WorkerFnType + 'static , { fn parse (input : ParseStream) -> syn :: Result < Self > { let parsed : Item = input . parse () ? ; let func = match parsed { Item :: Fn (m) => m , item => { return Err (syn :: Error :: new_spanned (item , format ! ("`{}` attribute can only be applied to functions" , F :: attr_name ()) ,)) } } ; let ItemFn { attrs , vis , sig , .. } = func . clone () ; if sig . generics . lifetimes () . next () . is_some () { return Err (syn :: Error :: new_spanned (sig . generics , format ! ("{} workers can't have generic lifetime parameters" , F :: worker_type_name ()) ,)) ; } if sig . constness . is_some () { return Err (syn :: Error :: new_spanned (sig . constness , format ! ("const functions can't be {} workers" , F :: worker_type_name ()) ,)) ; } if sig . abi . is_some () { return Err (syn :: Error :: new_spanned (sig . abi , format ! ("extern functions can't be {} workers" , F :: worker_type_name ()) ,)) ; } let recv_type = F :: parse_recv_type (& sig) ? ; let output_type = F :: parse_output_type (& sig) ? ; let is_async = sig . asyncness . is_some () ; Ok (Self { recv_type , output_type , generics : sig . generics , is_async , vis , attrs , name : sig . ident , worker_name : None , func , }) } }
};
}
