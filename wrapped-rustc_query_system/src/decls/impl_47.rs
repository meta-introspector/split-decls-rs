macro_rules! deps {
    () => {
        DepNodeColor!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl DepNodeColor { # [inline] fn is_green (self) -> bool { match self { DepNodeColor :: Red => false , DepNodeColor :: Green (_) => true , } } }
    };
}

impl_47!();