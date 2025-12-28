macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl std :: error :: Error for Error { }
    };
}

impl_14!()