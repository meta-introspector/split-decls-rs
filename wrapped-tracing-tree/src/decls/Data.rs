macro_rules! Data {
    () => {
        pub (crate) struct Data { start : Instant , kvs : Vec < (& 'static str , String) > , written : bool , }
    };
}

Data!();