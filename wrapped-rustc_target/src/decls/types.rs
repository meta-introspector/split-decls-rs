macro_rules! deps {
    () => {
        InlineAsmType!();
    };
}

macro_rules! types {
    () => {
        deps!();
        macro_rules ! types { ($ (_ : $ ($ ty : expr) ,+;) ? $ ($ feature : ident : $ ($ ty2 : expr) ,+;) *) => { { use super :: InlineAsmType ::*; & [$ ($ (($ ty , None) ,) *) ? $ ($ (($ ty2 , Some (rustc_span :: sym ::$ feature)) ,) *) *] } } ; }
    };
}

types!()