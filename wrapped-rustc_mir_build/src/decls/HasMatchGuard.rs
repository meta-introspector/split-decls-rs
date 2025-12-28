macro_rules! HasMatchGuard {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum HasMatchGuard { Yes , No , }
    };
}

HasMatchGuard!();