// Generated macro for RecoverableParser (trait)
macro_rules! Depcrate_parserRecoverableParser {
() => {
// Module: crate::parser
// Provides: {"RecoverableParser"}
// Dependencies: {}
# [doc = " Collect all errors when parsing the input"] # [doc = ""] # [doc = " [`Parser`]s will need to use [`Recoverable<I, _>`] for their input."] # [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] pub trait RecoverableParser < I , O , R , E > { # [doc = " Collect all errors when parsing the input"] # [doc = ""] # [doc = " If `self` fails, this acts like [`Parser::resume_after`] and returns `Ok(None)`."] # [doc = " Generally, this should be avoided by using"] # [doc = " [`Parser::retry_after`] and [`Parser::resume_after`] throughout your parser."] # [doc = ""] # [doc = " The empty `input` is returned to allow turning the errors into [`ParserError`]s."] fn recoverable_parse (& mut self , input : I) -> (I , Option < O > , Vec < R >) ; }
};
}
