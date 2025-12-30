// Generated macro for impl_62 (impl)
macro_rules! Depcrate_first_passimpl_62 {
() => {
// Module: crate::first_pass
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , (& 'src str , ApiStability) > for weedle :: interface :: ConstructorInterfaceMember < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , (self_name , stability) : (& 'src str , ApiStability) ,) -> Result < () > { let ident = weedle :: common :: Identifier (self_name) ; let non_null = weedle :: types :: MayBeNull { type_ : ident , q_mark : None , } ; let non_any = weedle :: types :: NonAnyType :: Identifier (non_null) ; let single = weedle :: types :: SingleType :: NonAny (non_any) ; let ty = weedle :: types :: Type :: Single (single) ; let return_ty = weedle :: types :: ReturnType :: Type (ty) ; first_pass_operation (record , FirstPassOperationType :: Interface , self_name , & [OperationId :: Constructor (Some (self_name))] , & self . args . body . list , & return_ty , & self . attributes , false , stability ,) ; Ok (()) } }
};
}
