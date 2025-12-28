macro_rules! deps {
    () => {
        Datetime!();
    };
}

macro_rules! Time {
    () => {
        deps!();
        # [doc = " A parsed TOML time value"] # [doc = ""] # [doc = " May be part of a [`Datetime`]. Alone, `Time` corresponds to a [Local Time]."] # [doc = " From the TOML v1.0.0 spec:"] # [doc = ""] # [doc = " > If you include only the time portion of an RFC 3339 formatted date-time,"] # [doc = " > it will represent that time of day without any relation to a specific"] # [doc = " > day or any offset or timezone."] # [doc = " >"] # [doc = " > ```toml"] # [doc = " > lt1 = 07:32:00"] # [doc = " > lt2 = 00:32:00.999999"] # [doc = " > ```"] # [doc = " >"] # [doc = " > Millisecond precision is required. Further precision of fractional"] # [doc = " > seconds is implementation-specific. If the value contains greater"] # [doc = " > precision than the implementation can support, the additional precision"] # [doc = " > must be truncated, not rounded."] # [doc = ""] # [doc = " [Local Time]: https://toml.io/en/v1.0.0#local-time"] # [derive (PartialEq , Eq , PartialOrd , Ord , Copy , Clone , Debug)] pub struct Time { # [doc = " Hour: 0 to 23"] pub hour : u8 , # [doc = " Minute: 0 to 59"] pub minute : u8 , # [doc = " Second: 0 to {58, 59, 60} (based on leap second rules)"] pub second : u8 , # [doc = " Nanosecond: 0 to `999_999_999`"] pub nanosecond : u32 , }
    };
}

Time!()