// Generated macro for impl_71 (impl)
macro_rules! Depcrate_first_passimpl_71 {
() => {
// Module: crate::first_pass
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , (& 'src str , ApiStability) > for weedle :: mixin :: OperationMixinMember < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , ctx : (& 'src str , ApiStability) ,) -> Result < () > { if self . stringifier . is_some () { log :: warn ! ("Unsupported webidl stringifier: {self:?}") ; return Ok (()) ; } first_pass_operation (record , FirstPassOperationType :: Mixin , ctx . 0 , & [OperationId :: Operation (self . identifier . map (| s | s . 0))] , & self . args . body . list , & self . return_type , & self . attributes , false , ctx . 1 ,) ; Ok (()) } }
};
}
