macro_rules! DatetimeKey {
    () => {
        # [cfg (feature = "serde")] struct DatetimeKey ;
    };
}

DatetimeKey!()