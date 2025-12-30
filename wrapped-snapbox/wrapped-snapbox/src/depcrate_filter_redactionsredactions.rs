// Generated macro for Redactions (struct)
macro_rules! Depcrate_filter_redactionsRedactions {
() => {
// Module: crate::filter::redactions
// Provides: {"Redactions"}
// Dependencies: {}
# [doc = " Replace data with placeholders"] # [doc = ""] # [doc = " This can be used for:"] # [doc = " - Handling test-run dependent data like temp directories or elapsed time"] # [doc = " - Making special characters more obvious (e.g. redacting a tab a `[TAB]`)"] # [doc = " - Normalizing platform-specific data like [`std::env::consts::EXE_SUFFIX`]"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut subst = snapbox::Redactions::new();"] # [doc = " subst.insert(\"[LOCATION]\", \"World\");"] # [doc = " assert_eq!(subst.redact(\"Hello World!\"), \"Hello [LOCATION]!\");"] # [doc = " ```"] # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct Redactions { vars : Option < std :: collections :: BTreeMap < RedactedValueInner , std :: collections :: BTreeSet < & 'static str > > , > , unused : Option < std :: collections :: BTreeSet < RedactedValueInner > > , }
};
}
