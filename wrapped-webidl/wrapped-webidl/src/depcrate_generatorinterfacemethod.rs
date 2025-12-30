// Generated macro for InterfaceMethod (struct)
macro_rules! Depcrate_generatorInterfaceMethod {
() => {
// Module: crate::generator
// Provides: {"InterfaceMethod"}
// Dependencies: {}
pub struct InterfaceMethod < 'a > { pub name : Ident , pub js_name : String , pub deprecated : Option < Option < String > > , pub arguments : Vec < (Ident , IdlType < 'a > , Type) > , pub variadic_type : Option < IdlType < 'a > > , pub ret_ty : Option < Type > , pub kind : InterfaceMethodKind , pub is_static : bool , pub structural : bool , pub catch : bool , pub variadic : bool , pub unstable : bool , }
};
}
