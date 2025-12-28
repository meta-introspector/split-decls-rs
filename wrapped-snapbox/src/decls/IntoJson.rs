macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! IntoJson {
    () => {
        deps!();
        # [doc = " Capture the serde representation of a value"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use snapbox::IntoJson as _;"] # [doc = ""] # [doc = " fn some_function() -> usize {"] # [doc = "     // ..."] # [doc = " # 5"] # [doc = " }"] # [doc = ""] # [doc = " let actual = some_function();"] # [doc = " let expected = snapbox::str![[\"5\"]];"] # [doc = " snapbox::assert_data_eq!(actual.into_json(), expected);"] # [doc = " ```"] # [cfg (feature = "json")] pub trait IntoJson { fn into_json (self) -> Data ; }
    };
}

IntoJson!()