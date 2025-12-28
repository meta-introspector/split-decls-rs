macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! DisplayValue {
    () => {
        deps!();
        # [doc = " A `Value` which serializes using `fmt::Display`."] # [doc = ""] # [doc = " Uses `record_debug` in the `Value` implementation to"] # [doc = " avoid an unnecessary evaluation."] # [derive (Clone)] pub struct DisplayValue < T : fmt :: Display > (T) ;
    };
}

DisplayValue!();