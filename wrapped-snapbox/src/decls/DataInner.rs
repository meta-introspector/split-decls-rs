macro_rules! deps {
    () => {
        DataError!();
        Error!();
    };
}

macro_rules! DataInner {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) enum DataInner { Error (DataError) , Binary (Vec < u8 >) , Text (String) , # [cfg (feature = "json")] Json (serde_json :: Value) , # [cfg (feature = "json")] JsonLines (serde_json :: Value) , # [cfg (feature = "term-svg")] TermSvg (String) , }
    };
}

DataInner!()