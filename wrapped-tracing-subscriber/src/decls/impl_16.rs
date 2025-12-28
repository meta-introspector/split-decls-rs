macro_rules! deps {
    () => {
        Delimited!();
        MakeVisitor!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < D , V > Delimited < D , V > { # [doc = " Returns a new [`MakeVisitor`] implementation that wraps `inner` so that"] # [doc = " it will format each visited field separated by the provided `delimiter`."] # [doc = ""] # [doc = " [`MakeVisitor`]: super::MakeVisitor"] pub fn new (delimiter : D , inner : V) -> Self { Self { delimiter , inner } } }
    };
}

impl_16!();