macro_rules! deps {
    () => {
        Interner!();
        IrPrint!();
    };
}

macro_rules! define_display_via_print {
    () => {
        deps!();
        macro_rules ! define_display_via_print { ($ ($ ty : ident) ,+ $ (,) ?) => { $ (impl < I : Interner > fmt :: Display for $ ty < I > { fn fmt (& self , fmt : & mut fmt :: Formatter <'_ >) -> fmt :: Result { < I as IrPrint <$ ty < I >>>:: print (self , fmt) } }) * } }
    };
}

define_display_via_print!()