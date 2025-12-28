macro_rules! RAW_VALUE_TOKEN {
    () => {
        # [doc = " The token used by `serde_json` to represent raw values."] # [doc = ""] # [doc = " It should be kept in sync with the following original until made public:"] # [doc = " https://github.com/serde-rs/json/blob/b48b9a3a0c09952579e98c8940fe0d1ee4aae588/src/raw.rs#L292"] # [cfg (feature = "raw_value")] pub const RAW_VALUE_TOKEN : & str = "$serde_json::private::RawValue" ;
    };
}

RAW_VALUE_TOKEN!();