// Generated macro for impl_66 (impl)
macro_rules! Depcrate_first_passimpl_66 {
() => {
// Module: crate::first_pass
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , (& 'src str , ApiStability) > for weedle :: interface :: AsyncIterableInterfaceMember < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , ctx : (& 'src str , ApiStability) ,) -> Result < () > { record . async_iterators . insert ("AsyncIterator") ; first_pass_operation (record , FirstPassOperationType :: Interface , ctx . 0 , & [OperationId :: Operation (Some ("entries"))] , & [] , & ReturnType :: Type (Type :: Single (SingleType :: NonAny (NonAnyType :: Identifier (MayBeNull { type_ : Identifier ("AsyncIterator") , q_mark : None , } ,)))) , & NEW_OBJECT_ATTR , false , ctx . 1 ,) ; first_pass_operation (record , FirstPassOperationType :: Interface , ctx . 0 , & [OperationId :: Operation (Some ("keys"))] , & [] , & ReturnType :: Type (Type :: Single (SingleType :: NonAny (NonAnyType :: Identifier (MayBeNull { type_ : Identifier ("AsyncIterator") , q_mark : None , } ,)))) , & NEW_OBJECT_ATTR , false , ctx . 1 ,) ; first_pass_operation (record , FirstPassOperationType :: Interface , ctx . 0 , & [OperationId :: Operation (Some ("values"))] , & [] , & ReturnType :: Type (Type :: Single (SingleType :: NonAny (NonAnyType :: Identifier (MayBeNull { type_ : Identifier ("AsyncIterator") , q_mark : None , } ,)))) , & NEW_OBJECT_ATTR , false , ctx . 1 ,) ; Ok (()) } }
};
}
