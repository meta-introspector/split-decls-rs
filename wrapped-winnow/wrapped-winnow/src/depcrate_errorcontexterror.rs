// Generated macro for ContextError (struct)
macro_rules! Depcrate_errorContextError {
() => {
// Module: crate::error
// Provides: {"ContextError"}
// Dependencies: {}
# [doc = " Accumulate context while backtracking errors"] # [doc = ""] # [doc = " See the [tutorial][crate::_tutorial::chapter_7#error-adaptation-and-rendering]"] # [doc = " for an example of how to adapt this to an application error with custom rendering."] # [derive (Debug)] pub struct ContextError < C = StrContext > { # [cfg (feature = "alloc")] context : alloc :: vec :: Vec < C > , # [cfg (not (feature = "alloc"))] context : core :: marker :: PhantomData < C > , # [cfg (feature = "std")] cause : Option < Box < dyn std :: error :: Error + Send + Sync + 'static > > , }
};
}
