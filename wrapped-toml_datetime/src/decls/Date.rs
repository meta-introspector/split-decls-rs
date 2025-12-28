macro_rules! deps {
    () => {
        Datetime!();
    };
}

macro_rules! Date {
    () => {
        deps!();
        # [doc = " A parsed TOML date value"] # [doc = ""] # [doc = " May be part of a [`Datetime`]. Alone, `Date` corresponds to a [Local Date]."] # [doc = " From the TOML v1.0.0 spec:"] # [doc = ""] # [doc = " > If you include only the date portion of an RFC 3339 formatted date-time,"] # [doc = " > it will represent that entire day without any relation to an offset or"] # [doc = " > timezone."] # [doc = " >"] # [doc = " > ```toml"] # [doc = " > ld1 = 1979-05-27"] # [doc = " > ```"] # [doc = ""] # [doc = " [Local Date]: https://toml.io/en/v1.0.0#local-date"] # [derive (PartialEq , Eq , PartialOrd , Ord , Copy , Clone , Debug)] pub struct Date { # [doc = " Year: four digits"] pub year : u16 , # [doc = " Month: 1 to 12"] pub month : u8 , # [doc = " Day: 1 to {28, 29, 30, 31} (based on month/year)"] pub day : u8 , }
    };
}

Date!()