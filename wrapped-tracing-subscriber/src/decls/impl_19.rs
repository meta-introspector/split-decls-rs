macro_rules! deps {
    () => {
        VisitDelimited!();
        VisitOutput!();
        VisitFmt!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < D , V > VisitOutput < fmt :: Result > for VisitDelimited < D , V > where V : VisitFmt , D : AsRef < str > , { fn finish (self) -> fmt :: Result { self . err ? ; self . inner . finish () } }
    };
}

impl_19!()