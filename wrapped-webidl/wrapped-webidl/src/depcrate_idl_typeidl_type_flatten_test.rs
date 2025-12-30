// Generated macro for idl_type_flatten_test (function)
macro_rules! Depcrate_idl_typeidl_type_flatten_test {
() => {
// Module: crate::idl_type
// Provides: {"idl_type_flatten_test"}
// Dependencies: {}
# [test] fn idl_type_flatten_test () { use self :: IdentifierType :: * ; use self :: IdlType :: * ; assert_eq ! (Union (vec ! [IdlType :: id ("Node" , Interface ("Node")) , Union (vec ! [Sequence (Box :: new (Long) ,) , IdlType :: id ("Event" , Interface ("Event"))]) , Nullable (Box :: new (Union (vec ! [IdlType :: id ("XMLHttpRequest" , Interface ("XMLHttpRequest")) , DomString ,])) ,) , Sequence (Box :: new (Union (vec ! [Sequence (Box :: new (Double) ,) , IdlType :: id ("NodeList" , Interface ("NodeList")) ,])) ,) ,]) . flatten (None) , vec ! [IdlType :: id ("Node" , Interface ("Node")) , Sequence (Box :: new (Long)) , IdlType :: id ("Event" , Interface ("Event")) , Nullable (Box :: new (IdlType :: id ("XMLHttpRequest" , Interface ("XMLHttpRequest")))) , Nullable (Box :: new (DomString)) , Sequence (Box :: new (Sequence (Box :: new (Double)))) , Sequence (Box :: new (IdlType :: id ("NodeList" , Interface ("NodeList")))) ,] ,) ; }
};
}
