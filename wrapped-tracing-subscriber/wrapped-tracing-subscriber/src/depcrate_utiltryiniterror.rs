// Generated macro for TryInitError (struct)
macro_rules! Depcrate_utilTryInitError {
() => {
// Module: crate::util
// Provides: {"TryInitError"}
// Dependencies: {}
# [doc = " Error returned by [`try_init`](SubscriberInitExt::try_init) if a global default subscriber could not be initialized."] pub struct TryInitError { # [cfg (feature = "std")] inner : Box < dyn Error + Send + Sync + 'static > , # [cfg (not (feature = "std"))] _p : () , }
};
}
