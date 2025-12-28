macro_rules! ImpliedFeatures {
    () => {
        type ImpliedFeatures = & 'static [& 'static str] ;
    };
}

ImpliedFeatures!();