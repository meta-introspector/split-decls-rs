// Generated macro for impl_423 (impl)
macro_rules! Depcrate_stream_recoverableimpl_423 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_423"}
// Dependencies: {}
impl < I , E , R > Recover < E > for Recoverable < I , R > where I : Stream , R : FromRecoverableError < Self , E > , R : core :: fmt :: Debug , E : crate :: error :: ParserError < Self > , { fn record_err (& mut self , token_start : & Self :: Checkpoint , err_start : & Self :: Checkpoint , err : E ,) -> Result < () , E > { if self . is_recoverable { if err . is_incomplete () { Err (err) } else { self . errors . push (R :: from_recoverable_error (token_start , err_start , self , err)) ; Ok (()) } } else { Err (err) } } # [doc = " Report whether the [`Stream`] can save off errors for recovery"] # [inline (always)] fn is_recovery_supported () -> bool { true } }
};
}
