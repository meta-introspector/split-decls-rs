macro_rules! deps {
    () => {
        Layer!();
        FilterFn!();
        Context!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < S , F > Layer < S > for FilterFn < F > where F : Fn (& Metadata < '_ >) -> bool + 'static , S : Subscriber , { fn enabled (& self , metadata : & Metadata < '_ > , _ : Context < '_ , S >) -> bool { self . is_enabled (metadata) } fn register_callsite (& self , metadata : & 'static Metadata < 'static >) -> Interest { self . is_callsite_enabled (metadata) } fn max_level_hint (& self) -> Option < LevelFilter > { self . max_level_hint } }
    };
}

impl_60!()