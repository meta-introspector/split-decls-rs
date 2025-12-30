// Generated macro for process_interface_attribute (function)
macro_rules! Depcrate_first_passprocess_interface_attribute {
() => {
// Module: crate::first_pass
// Provides: {"process_interface_attribute"}
// Dependencies: {}
fn process_interface_attribute < 'src > (record : & mut FirstPassRecord < 'src > , self_name : & 'src str , attr : & 'src ExtendedAttribute < 'src > ,) { let stability = record . interfaces [self_name] . stability ; let ident = weedle :: common :: Identifier (self_name) ; let non_null = weedle :: types :: MayBeNull { type_ : ident , q_mark : None , } ; let non_any = weedle :: types :: NonAnyType :: Identifier (non_null) ; let single = weedle :: types :: SingleType :: NonAny (non_any) ; let ty = weedle :: types :: Type :: Single (single) ; let return_ty = weedle :: types :: ReturnType :: Type (ty) ; match attr { ExtendedAttribute :: ArgList (list) if list . identifier . 0 == "Constructor" => { first_pass_operation (record , FirstPassOperationType :: Interface , self_name , & [OperationId :: Constructor (Some (self_name))] , & list . args . body . list , & return_ty , & None , false , stability ,) ; } ExtendedAttribute :: NoArgs (other) if (other . 0) . 0 == "Constructor" => { first_pass_operation (record , FirstPassOperationType :: Interface , self_name , & [OperationId :: Constructor (Some (self_name))] , & [] , & return_ty , & None , false , stability ,) ; } ExtendedAttribute :: NamedArgList (list) if list . lhs_identifier . 0 == "NamedConstructor" => { first_pass_operation (record , FirstPassOperationType :: Interface , self_name , & [OperationId :: NamedConstructor (IgnoreTraits (list . rhs_identifier . 0 ,))] , & list . args . body . list , & return_ty , & None , false , stability ,) ; } _ => { } } }
};
}
