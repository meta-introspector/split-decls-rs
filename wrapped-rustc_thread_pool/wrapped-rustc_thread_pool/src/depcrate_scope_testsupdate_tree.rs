// Generated macro for update_tree (function)
macro_rules! Depcrate_scope_testsupdate_tree {
() => {
// Module: crate::scope::tests
// Provides: {"update_tree"}
// Dependencies: {}
# [test] fn update_tree () { let mut tree : Tree < u32 > = random_tree (10) ; let values : Vec < u32 > = tree . iter () . cloned () . collect () ; tree . update (| v | * v += 1) ; let new_values : Vec < u32 > = tree . iter () . cloned () . collect () ; assert_eq ! (values . len () , new_values . len ()) ; for (& i , & j) in values . iter () . zip (& new_values) { assert_eq ! (i + 1 , j) ; } }
};
}
