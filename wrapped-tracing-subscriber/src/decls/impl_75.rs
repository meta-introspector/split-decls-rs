macro_rules! deps {
    () => {
        Layer!();
        Context!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < S : Subscriber > crate :: Layer < S > for LevelFilter { fn register_callsite (& self , metadata : & 'static Metadata < 'static >) -> Interest { if self >= metadata . level () { Interest :: always () } else { Interest :: never () } } fn enabled (& self , metadata : & Metadata < '_ > , _ : crate :: layer :: Context < '_ , S >) -> bool { self >= metadata . level () } fn max_level_hint (& self) -> Option < LevelFilter > { Some (* self) } }
    };
}

impl_75!();