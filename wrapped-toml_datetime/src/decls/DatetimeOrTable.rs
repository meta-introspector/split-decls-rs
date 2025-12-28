macro_rules! DatetimeOrTable {
    () => {
        struct DatetimeOrTable < 'm , 'de > { key : & 'm mut Option < alloc :: borrow :: Cow < 'de , str > > , }
    };
}

DatetimeOrTable!();