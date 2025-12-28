macro_rules! deps {
    () => {
        VisitOutput!();
        Alt!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < V , O > VisitOutput < O > for Alt < V > where V : VisitOutput < O > , { # [inline] fn finish (self) -> O { self . 0 . finish () } }
    };
}

impl_9!()