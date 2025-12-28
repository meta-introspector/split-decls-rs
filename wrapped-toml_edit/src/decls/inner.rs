macro_rules! deps {
    () => {
        ValueRepr!();
    };
}

macro_rules! inner {
    () => {
        deps!();
        # [cfg (not (feature = "display"))] mod inner { use super :: ValueRepr ; impl ValueRepr for String { } impl ValueRepr for i64 { } impl ValueRepr for f64 { } impl ValueRepr for bool { } impl ValueRepr for toml_datetime :: Datetime { } }
    };
}

inner!()