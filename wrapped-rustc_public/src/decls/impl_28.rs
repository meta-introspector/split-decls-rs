macro_rules! deps {
    () => {
        ValueAbi!();
        Scalar!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl ValueAbi { # [doc = " Returns `true` if the layout corresponds to an unsized type."] pub fn is_unsized (& self) -> bool { match * self { ValueAbi :: Scalar (_) | ValueAbi :: ScalarPair (..) | ValueAbi :: Vector { .. } => false , ValueAbi :: Aggregate { sized } => ! sized , } } }
    };
}

impl_28!()