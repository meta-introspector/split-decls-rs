// Generated macro for ABOUT_PERL_WORD (const)
macro_rules! Depcrate_appABOUT_PERL_WORD {
() => {
// Module: crate::app
// Provides: {"ABOUT_PERL_WORD"}
// Dependencies: {}
const ABOUT_PERL_WORD : & 'static str = "\
perl-word emits a table of codepoints in Unicode's definition of the \\w
character class, according to Annex C in UTS#18. In particular, this includes
the Alphabetic and Join_Control properties, in addition to the Decimal_Number,
Mark and Connector_Punctuation general categories.

Commands for \\s and \\d are not provided, since they directly correspond
to the property Whitespace and the general category Decimal_Number,
respectively.

The flags for this command are similar as the flags for property-bool.
" ;
};
}
