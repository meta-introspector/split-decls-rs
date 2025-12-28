macro_rules! deps {
    () => {
        Inline!();
    };
}

macro_rules! DataSourceInner {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum DataSourceInner { Path (std :: path :: PathBuf) , Inline (Inline) , }
    };
}

DataSourceInner!()