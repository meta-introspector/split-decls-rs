macro_rules! Build {
    () => {
        # [derive (Deserialize)] struct Build { # [serde (deserialize_with = "from_json")] features : Vec < String > , }
    };
}

Build!();