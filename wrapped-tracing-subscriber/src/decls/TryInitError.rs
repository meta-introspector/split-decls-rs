macro_rules! TryInitError {
    () => {
        # [doc = " Error returned by [`try_init`](SubscriberInitExt::try_init) if a global default subscriber could not be initialized."] pub struct TryInitError { # [cfg (feature = "std")] inner : Box < dyn Error + Send + Sync + 'static > , # [cfg (not (feature = "std"))] _p : () , }
    };
}

TryInitError!();