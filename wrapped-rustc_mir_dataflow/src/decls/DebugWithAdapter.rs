macro_rules! DebugWithAdapter {
    () => {
        # [doc = " Implements `fmt::Debug` by deferring to `<T as DebugWithContext<C>>::fmt_with`."] pub struct DebugWithAdapter < 'a , T , C > { pub this : T , pub ctxt : & 'a C , }
    };
}

DebugWithAdapter!();