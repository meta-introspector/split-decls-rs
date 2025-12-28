macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! ToDebug {
    () => {
        deps!();
        # [doc = " Capture the pretty debug representation of a value"] # [doc = ""] # [doc = " Note: this is fairly brittle as debug representations are not generally subject to semver"] # [doc = " guarantees."] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use snapbox::ToDebug as _;"] # [doc = ""] # [doc = " fn some_function() -> usize {"] # [doc = "     // ..."] # [doc = " # 5"] # [doc = " }"] # [doc = ""] # [doc = " let actual = some_function();"] # [doc = " let expected = snapbox::str![[\"5\"]];"] # [doc = " snapbox::assert_data_eq!(actual.to_debug(), expected);"] # [doc = " ```"] pub trait ToDebug { fn to_debug (& self) -> Data ; }
    };
}

ToDebug!()