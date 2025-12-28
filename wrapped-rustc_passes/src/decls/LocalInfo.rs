macro_rules! LocalInfo {
    () => {
        # [derive (Copy , Clone , Debug)] struct LocalInfo { id : HirId , name : Symbol , is_shorthand : bool , }
    };
}

LocalInfo!();