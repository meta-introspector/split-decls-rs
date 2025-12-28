macro_rules! impl_82 {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " Trivially safe, as `PhantomNotSend` doesn't have any API."] unsafe impl Sync for PhantomNotSend { }
    };
}

impl_82!();