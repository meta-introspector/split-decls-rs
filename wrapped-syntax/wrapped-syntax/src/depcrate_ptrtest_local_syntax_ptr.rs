// Generated macro for test_local_syntax_ptr (function)
macro_rules! Depcrate_ptrtest_local_syntax_ptr {
() => {
// Module: crate::ptr
// Provides: {"test_local_syntax_ptr"}
// Dependencies: {}
# [test] fn test_local_syntax_ptr () { use crate :: { AstNode , SourceFile , ast } ; let file = SourceFile :: parse ("struct Foo { f: u32, }" , parser :: Edition :: CURRENT) . ok () . unwrap () ; let field = file . syntax () . descendants () . find_map (ast :: RecordField :: cast) . unwrap () ; let ptr = SyntaxNodePtr :: new (field . syntax ()) ; let field_syntax = ptr . to_node (file . syntax ()) ; assert_eq ! (field . syntax () , & field_syntax) ; }
};
}
