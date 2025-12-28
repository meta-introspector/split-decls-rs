macro_rules! deps {
    () => {
        DynFilterFn!();
        FilterFn!();
        Context!();
    };
}

macro_rules! macro_70 {
    () => {
        deps!();
        feature ! { #! [all (feature = "registry" , feature = "std")] use crate :: layer :: Filter ; impl < S , F > Filter < S > for FilterFn < F > where F : Fn (& Metadata <'_ >) -> bool , { fn enabled (& self , metadata : & Metadata <'_ >, _ : & Context <'_ , S >) -> bool { self . is_enabled (metadata) } fn callsite_enabled (& self , metadata : &'static Metadata <'static >) -> Interest { self . is_callsite_enabled (metadata) } fn max_level_hint (& self) -> Option < LevelFilter > { self . max_level_hint } } impl < S , F , R > Filter < S > for DynFilterFn < S , F , R > where F : Fn (& Metadata <'_ >, & Context <'_ , S >) -> bool , R : Fn (&'static Metadata <'static >) -> Interest , { fn enabled (& self , metadata : & Metadata <'_ >, cx : & Context <'_ , S >) -> bool { self . is_enabled (metadata , cx) } fn callsite_enabled (& self , metadata : &'static Metadata <'static >) -> Interest { self . is_callsite_enabled (metadata) } fn max_level_hint (& self) -> Option < LevelFilter > { self . max_level_hint } } }
    };
}

macro_70!()