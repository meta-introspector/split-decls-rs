macro_rules! has_been_set {
    () => {
        # [doc = " Returns true if a `tracing` dispatcher has ever been set."] # [doc = ""] # [doc = " This may be used to completely elide trace points if tracing is not in use"] # [doc = " at all or has yet to be initialized."] # [doc (hidden)] # [inline (always)] pub fn has_been_set () -> bool { EXISTS . load (Ordering :: Relaxed) }
    };
}

has_been_set!()