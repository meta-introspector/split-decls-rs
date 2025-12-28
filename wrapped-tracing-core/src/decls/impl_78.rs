macro_rules! deps {
    () => {
        SetGlobalDefaultError!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl error :: Error for SetGlobalDefaultError { }
    };
}

impl_78!()