macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! DataFormat {
    () => {
        deps!();
        # [doc = " Describes the structure of [`Data`][crate::Data]"] # [derive (Clone , Debug , PartialEq , Eq , Copy , Hash , Default)] # [non_exhaustive] pub enum DataFormat { # [doc = " Processing of the [`Data`][crate::Data] failed"] Error , # [doc = " Non-textual, opaque data"] Binary , # [default] Text , # [cfg (feature = "json")] Json , # [doc = " Streamed JSON output according to <https://jsonlines.org/>"] # [cfg (feature = "json")] JsonLines , # [doc = " [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code#DOS_and_Windows)"] # [doc = " rendered as [svg](https://docs.rs/anstyle-svg)"] # [cfg (feature = "term-svg")] TermSvg , }
    };
}

DataFormat!();