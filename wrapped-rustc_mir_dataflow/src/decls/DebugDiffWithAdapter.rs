macro_rules! DebugDiffWithAdapter {
    () => {
        # [doc = " Implements `fmt::Debug` by deferring to `<T as DebugWithContext<C>>::fmt_diff_with`."] pub struct DebugDiffWithAdapter < 'a , T , C > { pub new : T , pub old : T , pub ctxt : & 'a C , }
    };
}

DebugDiffWithAdapter!();