macro_rules! deps {
    () => {
        ParseLevelError!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl std :: error :: Error for ParseLevelError { }
    };
}

impl_201!();