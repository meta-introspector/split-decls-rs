macro_rules! __FORCE {
    () => {
        # [doc (hidden)] pub const __FORCE : bool = cfg ! (feature = "force-always-assert") ;
    };
}

__FORCE!()