macro_rules! deps {
    () => {
        Alt!();
        VisitFmt!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < V > VisitFmt for Alt < V > where V : VisitFmt , { # [inline] fn writer (& mut self) -> & mut dyn fmt :: Write { self . 0 . writer () } }
    };
}

impl_11!()