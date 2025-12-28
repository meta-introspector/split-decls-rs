macro_rules! NoDrop {
    () => {
        # [repr (transparent)] pub (crate) struct NoDrop < T : ? Sized > (ManuallyDrop < T >) ;
    };
}

NoDrop!();