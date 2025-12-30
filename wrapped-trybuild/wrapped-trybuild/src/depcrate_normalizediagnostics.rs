// Generated macro for diagnostics (function)
macro_rules! Depcrate_normalizediagnostics {
() => {
// Module: crate::normalize
// Provides: {"diagnostics"}
// Dependencies: {}
# [doc = " For a given compiler output, produces the set of saved outputs against which"] # [doc = " the compiler's output would be considered correct. If the test's saved"] # [doc = " stderr file is identical to any one of these variations, the test will pass."] # [doc = ""] # [doc = " This is a set rather than just one normalized output in order to avoid"] # [doc = " breaking existing tests when introducing new normalization steps. Someone"] # [doc = " may have saved stderr snapshots with an older version of trybuild, and those"] # [doc = " tests need to continue to pass with newer versions of trybuild."] # [doc = ""] # [doc = " There is one \"preferred\" variation which is what we print when the stderr"] # [doc = " file is absent or not a match."] pub (crate) fn diagnostics (output : & str , context : Context) -> Variations { let output = output . replace ("\r\n" , "\n") ; let mut result = Variations :: default () ; for (i , normalization) in Normalization :: ALL . iter () . enumerate () { result . variations [i] = apply (& output , * normalization , context) ; } result }
};
}
