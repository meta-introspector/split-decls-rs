macro_rules! as_ref {
    () => {
        # [cfg (feature = "fs")] pub (crate) mod as_ref ;
    };
}

as_ref!()