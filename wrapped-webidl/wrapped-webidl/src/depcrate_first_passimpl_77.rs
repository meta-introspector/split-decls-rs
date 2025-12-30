// Generated macro for impl_77 (impl)
macro_rules! Depcrate_first_passimpl_77 {
() => {
// Module: crate::first_pass
// Provides: {"impl_77"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , (& 'src str , ApiStability) > for weedle :: namespace :: OperationNamespaceMember < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , (self_name , stability) : (& 'src str , ApiStability) ,) -> Result < () > { first_pass_operation (record , FirstPassOperationType :: Namespace , self_name , & [OperationId :: Operation (self . identifier . map (| s | s . 0))] , & self . args . body . list , & self . return_type , & self . attributes , true , stability ,) ; Ok (()) } }
};
}
