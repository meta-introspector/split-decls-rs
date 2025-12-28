macro_rules! deps {
    () => {
        VisitFmt!();
        VisitDelimited!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < D , V > VisitFmt for VisitDelimited < D , V > where V : VisitFmt , D : AsRef < str > , { fn writer (& mut self) -> & mut dyn fmt :: Write { self . inner . writer () } }
    };
}

impl_20!()