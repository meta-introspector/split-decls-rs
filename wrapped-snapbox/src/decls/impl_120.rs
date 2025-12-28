macro_rules! deps {
    () => {
        IntoJson!();
        Data!();
        DataFormat!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        # [cfg (feature = "json")] impl < S : serde :: Serialize > IntoJson for S { fn into_json (self) -> Data { match serde_json :: to_value (self) { Ok (value) => Data :: json (value) , Err (err) => Data :: error (err . to_string () , DataFormat :: Json) , } } }
    };
}

impl_120!()