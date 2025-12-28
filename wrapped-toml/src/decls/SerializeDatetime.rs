macro_rules! SerializeDatetime {
    () => {
        # [doc (hidden)] pub struct SerializeDatetime < 'd > { dst : & 'd mut String , inner : toml_datetime :: ser :: DatetimeSerializer , }
    };
}

SerializeDatetime!()