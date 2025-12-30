// Generated macro for param_names (function)
macro_rules! Depcrate_expandparam_names {
() => {
// Module: crate::expand
// Provides: {"param_names"}
// Dependencies: {}
fn param_names (pat : Pat , record_type : RecordType) -> Box < dyn Iterator < Item = (Ident , RecordType) > > { match pat { Pat :: Ident (PatIdent { ident , .. }) => Box :: new (iter :: once ((ident , record_type))) , Pat :: Reference (PatReference { pat , .. }) => param_names (* pat , record_type) , Pat :: Struct (PatStruct { fields , .. }) => Box :: new (fields . into_iter () . flat_map (| FieldPat { pat , .. } | param_names (* pat , RecordType :: Debug)) ,) , Pat :: Tuple (PatTuple { elems , .. }) => Box :: new (elems . into_iter () . flat_map (| p | param_names (p , RecordType :: Debug)) ,) , Pat :: TupleStruct (PatTupleStruct { elems , .. }) => Box :: new (elems . into_iter () . flat_map (| p | param_names (p , RecordType :: Debug)) ,) , _ => Box :: new (iter :: empty ()) , } }
};
}
