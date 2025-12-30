// Generated macro for CanonicalizeMode (enum)
macro_rules! Depcrate_canonicalizerCanonicalizeMode {
() => {
// Module: crate::canonicalizer
// Provides: {"CanonicalizeMode"}
// Dependencies: {}
# [doc = " Whether we're canonicalizing a query input or the query response."] # [doc = ""] # [doc = " When canonicalizing an input we're in the context of the caller"] # [doc = " while canonicalizing the response happens in the context of the"] # [doc = " query."] # [derive (Debug , Clone , Copy)] enum CanonicalizeMode { Input (CanonicalizeInputKind) , # [doc = " FIXME: We currently return region constraints referring to"] # [doc = " placeholders and inference variables from a binder instantiated"] # [doc = " inside of the query."] # [doc = ""] # [doc = " In the long term we should eagerly deal with these constraints"] # [doc = " inside of the query and only propagate constraints which are"] # [doc = " actually nameable by the caller."] Response { # [doc = " The highest universe nameable by the caller."] # [doc = ""] # [doc = " All variables in a universe nameable by the caller get mapped"] # [doc = " to the root universe in the response and then mapped back to"] # [doc = " their correct universe when applying the query response in the"] # [doc = " context of the caller."] # [doc = ""] # [doc = " This doesn't work for universes created inside of the query so"] # [doc = " we do remember their universe in the response."] max_input_universe : ty :: UniverseIndex , } , }
};
}
