macro_rules! deps {
    () => {
        Nothing!();
    };
}

macro_rules! impl_536 {
    () => {
        deps!();
        # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Eq for Nothing { }
    };
}

impl_536!()