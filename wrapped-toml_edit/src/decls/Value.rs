macro_rules! deps {
    () => {
        Array!();
        Formatted!();
        InlineTable!();
    };
}

macro_rules! Value {
    () => {
        deps!();
        # [doc = " For [`Key`]/Value pairs under a [`Table`][crate::Table] header or inside another"] # [doc = " Value"] # [derive (Debug , Clone)] pub enum Value { # [doc = " A string value."] String (Formatted < String >) , # [doc = " A 64-bit integer value."] Integer (Formatted < i64 >) , # [doc = " A 64-bit float value."] Float (Formatted < f64 >) , # [doc = " A boolean value."] Boolean (Formatted < bool >) , # [doc = " An RFC 3339 formatted date-time with offset."] Datetime (Formatted < Datetime >) , # [doc = " An inline array of values."] Array (Array) , # [doc = " An inline table of key/value pairs."] InlineTable (InlineTable) , }
    };
}

Value!();