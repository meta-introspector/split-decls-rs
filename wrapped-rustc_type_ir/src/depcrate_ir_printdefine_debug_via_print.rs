// Generated macro for define_debug_via_print (macro)
macro_rules! Depcrate_ir_printdefine_debug_via_print {
() => {
// Module: crate::ir_print
// Provides: {"define_debug_via_print"}
// Dependencies: {}
macro_rules ! define_debug_via_print { ($ ($ ty : ident) ,+ $ (,) ?) => { $ (impl < I : Interner > fmt :: Debug for $ ty < I > { fn fmt (& self , fmt : & mut fmt :: Formatter <'_ >) -> fmt :: Result { < I as IrPrint <$ ty < I >>>:: print_debug (self , fmt) } }) * } }
};
}
