macro_rules! deps {
    () => {
        WithSubscriber!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < T : Sized > WithSubscriber for T { }
    };
}

impl_49!()