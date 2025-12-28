macro_rules! deps {
    () => {
        SerdeAdapter!();
        RealSerdeAdapter!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl SerdeAdapter for RealSerdeAdapter { fn to_string_pretty < T : ? Sized + Debug + Serialize > (& self , value : & T) -> Result < String , String > { serde_json :: to_string_pretty (value) . map_err (| e | format ! ("Failed to serialize to JSON: {}" , e)) } fn from_str < 'a , T : Debug + Deserialize < 'a > > (& self , s : & 'a str) -> Result < T , String > { serde_json :: from_str (s) . map_err (| e | format ! ("Failed to deserialize from JSON: {}" , e)) } }
    };
}

impl_17!()