macro_rules! Optimizations {
    () => {
        # [doc = " Whether to allow non-[required] optimizations"] # [doc = ""] # [doc = " [required]: MirPass::is_required"] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum Optimizations { Suppressed , Allowed , }
    };
}

Optimizations!()