// Generated macro for Interface (struct)
macro_rules! Depcrate_generatorInterface {
() => {
// Module: crate::generator
// Provides: {"Interface"}
// Dependencies: {}
pub struct Interface < 'a > { pub name : Ident , pub js_name : String , pub deprecated : Option < Option < String > > , pub has_interface : bool , pub parents : Vec < Ident > , pub consts : Vec < Const > , pub attributes : Vec < InterfaceAttribute > , pub methods : Vec < InterfaceMethod < 'a > > , pub unstable : bool , }
};
}
