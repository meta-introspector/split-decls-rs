// Generated macro for impl_170 (impl)
macro_rules! Depcrate_parserimpl_170 {
() => {
// Module: crate::parser
// Provides: {"impl_170"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < P , I , O , R , E > RecoverableParser < I , O , R , E > for P where P : Parser < Recoverable < I , R > , O , E > , I : Stream , I : StreamIsPartial , R : FromRecoverableError < Recoverable < I , R > , E > , R : core :: fmt :: Debug , E : FromRecoverableError < Recoverable < I , R > , E > , E : ParserError < Recoverable < I , R > > , E : core :: fmt :: Debug , { fn recoverable_parse (& mut self , input : I) -> (I , Option < O > , Vec < R >) { debug_assert ! (! I :: is_partial_supported () , "partial streams need to handle `ErrMode::Incomplete`") ; let start = input . checkpoint () ; let mut input = Recoverable :: new (input) ; let start_token = input . checkpoint () ; let result = (self . by_ref () , crate :: combinator :: eof . resume_after (crate :: token :: rest . void ()) ,) . parse_next (& mut input) ; let (o , err) = match result { Ok ((o , _)) => (Some (o) , None) , Err (err) => { let err_start = input . checkpoint () ; let err = R :: from_recoverable_error (& start_token , & err_start , & input , err) ; (None , Some (err)) } } ; let (mut input , mut errs) = input . into_parts () ; input . reset (& start) ; if let Some (err) = err { errs . push (err) ; } (input , o , errs) } }
};
}
