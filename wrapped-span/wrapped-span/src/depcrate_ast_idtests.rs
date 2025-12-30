// Generated macro for tests (module)
macro_rules! Depcrate_ast_idtests {
() => {
// Module: crate::ast_id
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use syntax :: { AstNode , Edition , SourceFile , SyntaxKind , SyntaxNodePtr , WalkEvent , ast } ; use crate :: AstIdMap ; # [test] fn check_all_nodes () { let syntax = SourceFile :: parse (r#"
extern crate foo;
fn foo() {
    union U {}
}
struct S;
macro_rules! m {}
macro m2() {}
trait Trait {}
impl Trait for S {}
impl S {}
impl m!() {}
impl m2!() for m!() {}
type T = i32;
enum E {
    V1(),
    V2 {},
    V3,
}
struct S; // duplicate
extern "C" {
    static S: i32;
}
static mut S: i32 = 0;
const FOO: i32 = 0;
        "# , Edition :: CURRENT ,) . syntax_node () ; let ast_id_map = AstIdMap :: from_source (& syntax) ; for node in syntax . preorder () { let WalkEvent :: Enter (node) = node else { continue } ; if ! matches ! (node . kind () , SyntaxKind :: EXTERN_CRATE | SyntaxKind :: FN | SyntaxKind :: UNION | SyntaxKind :: STRUCT | SyntaxKind :: MACRO_RULES | SyntaxKind :: MACRO_DEF | SyntaxKind :: MACRO_CALL | SyntaxKind :: TRAIT | SyntaxKind :: IMPL | SyntaxKind :: TYPE_ALIAS | SyntaxKind :: ENUM | SyntaxKind :: VARIANT | SyntaxKind :: EXTERN_BLOCK | SyntaxKind :: STATIC | SyntaxKind :: CONST) { continue ; } let ptr = SyntaxNodePtr :: new (& node) ; let ast_id = ast_id_map . erased_ast_id (ptr) ; let turn_back = ast_id_map . get_erased (ast_id) ; assert_eq ! (ptr , turn_back) ; } } # [test] fn different_names_get_different_hashes () { let syntax = SourceFile :: parse (r#"
fn foo() {}
fn bar() {}
        "# , Edition :: CURRENT ,) . syntax_node () ; let ast_id_map = AstIdMap :: from_source (& syntax) ; let fns = syntax . descendants () . filter_map (ast :: Fn :: cast) . collect :: < Vec < _ > > () ; let [foo_fn , bar_fn] = fns . as_slice () else { panic ! ("not exactly 2 functions") ; } ; let foo_fn_id = ast_id_map . ast_id (foo_fn) ; let bar_fn_id = ast_id_map . ast_id (bar_fn) ; assert_ne ! (foo_fn_id . raw . hash_value () , bar_fn_id . raw . hash_value () , "hashes are equal") ; } # [test] fn different_parents_get_different_hashes () { let syntax = SourceFile :: parse (r#"
fn foo() {
    m!();
}
fn bar() {
    m!();
}
        "# , Edition :: CURRENT ,) . syntax_node () ; let ast_id_map = AstIdMap :: from_source (& syntax) ; let macro_calls = syntax . descendants () . filter_map (ast :: MacroCall :: cast) . collect :: < Vec < _ > > () ; let [macro_call_foo , macro_call_bar] = macro_calls . as_slice () else { panic ! ("not exactly 2 macro calls") ; } ; let macro_call_foo_id = ast_id_map . ast_id (macro_call_foo) ; let macro_call_bar_id = ast_id_map . ast_id (macro_call_bar) ; assert_ne ! (macro_call_foo_id . raw . hash_value () , macro_call_bar_id . raw . hash_value () , "hashes are equal") ; } # [test] fn blocks_with_no_items_have_no_id () { let syntax = SourceFile :: parse (r#"
fn foo() {
    let foo = 1;
    bar(foo);
}
        "# , Edition :: CURRENT ,) . syntax_node () ; let ast_id_map = AstIdMap :: from_source (& syntax) ; let block = syntax . descendants () . find_map (ast :: BlockExpr :: cast) . expect ("no block") ; assert ! (ast_id_map . ast_id_for_block (& block) . is_none ()) ; } }
};
}
