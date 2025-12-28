macro_rules! SerializeDatetime {
    () => {
        # [doc (hidden)] pub struct SerializeDatetime { inner : toml_datetime :: ser :: DatetimeSerializer , }
    };
}

SerializeDatetime!();