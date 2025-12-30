// Generated macro for Function (struct)
macro_rules! Depcrate_generatorFunction {
() => {
// Module: crate::generator
// Provides: {"Function"}
// Dependencies: {}
pub struct Function < 'a > { pub name : Ident , pub js_name : String , pub arguments : Vec < (Ident , IdlType < 'a > , Type) > , pub ret_ty : Option < Type > , pub catch : bool , pub variadic : bool , pub unstable : bool , }
};
}
