macro_rules! lints {
    () => {
        # [cfg (feature = "rustc")] pub (crate) mod lints ;
    };
}

lints!();