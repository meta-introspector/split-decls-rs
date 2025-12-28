macro_rules! deps {
    () => {
        SerializeDatetime!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl SerializeDatetime { pub (crate) fn new () -> Self { Self { inner : toml_datetime :: ser :: DatetimeSerializer :: new () , } } }
    };
}

impl_362!()