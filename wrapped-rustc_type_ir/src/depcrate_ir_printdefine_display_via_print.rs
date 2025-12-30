// Generated macro for define_display_via_print (macro)
macro_rules! Depcrate_ir_printdefine_display_via_print {
() => {
// Module: crate::ir_print
// Provides: {"define_display_via_print"}
// Dependencies: {}
macro_rules ! define_display_via_print { ($ ($ ty : ident) ,+ $ (,) ?) => { $ (impl < I : Interner > fmt :: Display for $ ty < I > { fn fmt (& self , fmt : & mut fmt :: Formatter <'_ >) -> fmt :: Result { < I as IrPrint <$ ty < I >>>:: print (self , fmt) } }) * } }
};
}
