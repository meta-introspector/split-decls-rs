// Generated macro for MacroKind (enum)
macro_rules! Depcrate_hygieneMacroKind {
() => {
// Module: crate::hygiene
// Provides: {"MacroKind"}
// Dependencies: {}
# [doc = " The kind of macro invocation or definition."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Encodable , Decodable , Hash , Debug)] # [derive (HashStable_Generic)] pub enum MacroKind { # [doc = " A bang macro `foo!()`."] Bang , # [doc = " An attribute macro `#[foo]`."] Attr , # [doc = " A derive macro `#[derive(Foo)]`"] Derive , }
};
}
