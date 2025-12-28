macro_rules! deps {
    () => {
        VisitFmt!();
        VisitDelimited!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < D , V > VisitDelimited < D , V > { # [doc = " Returns a new [`Visit`] implementation that wraps `inner` so that"] # [doc = " each formatted field is separated by the provided `delimiter`."] # [doc = ""] # [doc = " [`Visit`]: tracing_core::field::Visit"] pub fn new (delimiter : D , inner : V) -> Self { Self { delimiter , inner , seen : false , err : Ok (()) , } } fn delimit (& mut self) where V : VisitFmt , D : AsRef < str > , { if self . err . is_err () { return ; } if self . seen { self . err = self . inner . writer () . write_str (self . delimiter . as_ref ()) ; } self . seen = true ; } }
    };
}

impl_17!();