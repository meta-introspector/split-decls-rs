macro_rules! deps {
    () => {
        VisitOutput!();
        Messages!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < V , O > VisitOutput < O > for Messages < V > where V : VisitOutput < O > , { # [inline] fn finish (self) -> O { self . 0 . finish () } }
    };
}

impl_27!();