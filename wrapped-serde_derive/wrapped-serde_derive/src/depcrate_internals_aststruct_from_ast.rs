// Generated macro for struct_from_ast (function)
macro_rules! Depcrate_internals_aststruct_from_ast {
() => {
// Module: crate::internals::ast
// Provides: {"struct_from_ast"}
// Dependencies: {}
fn struct_from_ast < 'a > (cx : & Ctxt , fields : & 'a syn :: Fields , attrs : Option < & attr :: Variant > , container_default : & attr :: Default , private : & Ident ,) -> (Style , Vec < Field < 'a > >) { match fields { syn :: Fields :: Named (fields) => (Style :: Struct , fields_from_ast (cx , & fields . named , attrs , container_default , private) ,) , syn :: Fields :: Unnamed (fields) if fields . unnamed . len () == 1 => (Style :: Newtype , fields_from_ast (cx , & fields . unnamed , attrs , container_default , private) ,) , syn :: Fields :: Unnamed (fields) => (Style :: Tuple , fields_from_ast (cx , & fields . unnamed , attrs , container_default , private) ,) , syn :: Fields :: Unit => (Style :: Unit , Vec :: new ()) , } }
};
}
