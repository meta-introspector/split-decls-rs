macro_rules! deps {
    () => {
        Interner!();
        IrPrint!();
    };
}

macro_rules! define_debug_via_print {
    () => {
        deps!();
        macro_rules ! define_debug_via_print { ($ ($ ty : ident) ,+ $ (,) ?) => { $ (impl < I : Interner > fmt :: Debug for $ ty < I > { fn fmt (& self , fmt : & mut fmt :: Formatter <'_ >) -> fmt :: Result { < I as IrPrint <$ ty < I >>>:: print_debug (self , fmt) } }) * } }
    };
}

define_debug_via_print!();