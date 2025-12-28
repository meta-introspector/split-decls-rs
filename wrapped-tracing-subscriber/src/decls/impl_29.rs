macro_rules! deps {
    () => {
        VisitFmt!();
        Messages!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < V > VisitFmt for Messages < V > where V : VisitFmt , { # [inline] fn writer (& mut self) -> & mut dyn fmt :: Write { self . 0 . writer () } }
    };
}

impl_29!();