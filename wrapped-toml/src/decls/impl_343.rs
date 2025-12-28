macro_rules! deps {
    () => {
        SerializeDatetime!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < 'd > SerializeDatetime < 'd > { pub (crate) fn new (dst : & 'd mut String) -> Self { Self { dst , inner : toml_datetime :: ser :: DatetimeSerializer :: new () , } } }
    };
}

impl_343!();