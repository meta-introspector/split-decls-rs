macro_rules! deps {
    () => {
        DenseLocationMap!();
    };
}

macro_rules! macro_219 {
    () => {
        deps!();
        rustc_index :: newtype_index ! { # [doc = " A single integer representing a `Location` in the MIR control-flow"] # [doc = " graph. Constructed efficiently from `DenseLocationMap`."] # [orderable] # [debug_format = "PointIndex({})"] pub struct PointIndex { } }
    };
}

macro_219!()