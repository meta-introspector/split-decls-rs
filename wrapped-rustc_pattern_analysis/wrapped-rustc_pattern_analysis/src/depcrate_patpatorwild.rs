// Generated macro for PatOrWild (enum)
macro_rules! Depcrate_patPatOrWild {
() => {
// Module: crate::pat
// Provides: {"PatOrWild"}
// Dependencies: {}
# [doc = " Represents either a pattern obtained from user input or a wildcard constructed during the"] # [doc = " algorithm. Do not use `Wild` to represent a wildcard pattern comping from user input."] # [doc = ""] # [doc = " This is morally `Option<&'p DeconstructedPat>` where `None` is interpreted as a wildcard."] pub (crate) enum PatOrWild < 'p , Cx : PatCx > { # [doc = " A non-user-provided wildcard, created during specialization."] Wild , # [doc = " A user-provided pattern."] Pat (& 'p DeconstructedPat < Cx >) , }
};
}
