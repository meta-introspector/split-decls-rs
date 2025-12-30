// Generated macro for impl_61 (impl)
macro_rules! Depcrate_first_passimpl_61 {
() => {
// Module: crate::first_pass
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , (& 'src str , ApiStability) > for weedle :: interface :: OperationInterfaceMember < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , (self_name , stability) : (& 'src str , ApiStability) ,) -> Result < () > { let is_static = match self . modifier { Some (StringifierOrStatic :: Stringifier (_)) => { log :: warn ! ("Unsupported webidl stringifier: {self:?}") ; return Ok (()) ; } Some (StringifierOrStatic :: Static (_)) => true , None => false , } ; let mut ids = vec ! [OperationId :: Operation (self . identifier . map (| s | s . 0))] ; if let Some (special) = self . special { match special { Special :: Getter (_) => ids . push (OperationId :: IndexingGetter) , Special :: Setter (_) => ids . push (OperationId :: IndexingSetter) , Special :: Deleter (_) => ids . push (OperationId :: IndexingDeleter) , Special :: LegacyCaller (_) => { } } ; } first_pass_operation (record , FirstPassOperationType :: Interface , self_name , & ids , & self . args . body . list , & self . return_type , & self . attributes , is_static , stability ,) ; Ok (()) } }
};
}
