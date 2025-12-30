// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl Render for TypeRef { fn render (& self , src : & mut String) { match self { TypeRef :: Name (t) => { src . push_str (& t . name . as_str () . to_camel_case ()) ; if let Type :: List (_) = & * * t . type_ () { src . push_str ("<'_>") ; } } TypeRef :: Value (v) => match & * * v { Type :: Builtin (t) => t . render (src) , Type :: List (t) => match & * * t . type_ () { Type :: Builtin (BuiltinType :: Char) => src . push_str ("&str") , _ => { src . push_str ("&'a [") ; t . render (src) ; src . push_str ("]") ; } } , Type :: Pointer (t) => { src . push_str ("*mut ") ; t . render (src) ; } Type :: ConstPointer (t) => { src . push_str ("*const ") ; t . render (src) ; } Type :: Variant (v) if v . is_bool () => src . push_str ("bool") , Type :: Variant (v) => match v . as_expected () { Some ((ok , err)) => { src . push_str ("Result<") ; match ok { Some (ty) => ty . render (src) , None => src . push_str ("()") , } src . push_str (",") ; match err { Some (ty) => ty . render (src) , None => src . push_str ("()") , } src . push_str (">") ; } None => { panic ! ("unsupported anonymous variant") } } , Type :: Record (r) if r . is_tuple () => { src . push_str ("(") ; for member in r . members . iter () { member . tref . render (src) ; src . push_str (",") ; } src . push_str (")") ; } t => panic ! ("reference to anonymous {} not possible!" , t . kind ()) , } , } } }
};
}
