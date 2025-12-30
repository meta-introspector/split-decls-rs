// Generated macro for skip_current_dir (function)
macro_rules! Depcrate_tests_recursiveskip_current_dir {
() => {
// Module: crate::tests::recursive
// Provides: {"skip_current_dir"}
// Dependencies: {}
# [test] fn skip_current_dir () { let dir = Dir :: tmp () ; dir . mkdirp ("foo/bar/baz") ; dir . mkdirp ("quux") ; let mut paths = vec ! [] ; let mut it = WalkDir :: new (dir . path ()) . into_iter () ; while let Some (result) = it . next () { let ent = result . unwrap () ; paths . push (ent . path () . to_path_buf ()) ; if ent . file_name () == "bar" { it . skip_current_dir () ; } } paths . sort () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("foo") , dir . join ("foo") . join ("bar") , dir . join ("quux") ,] ; assert_eq ! (expected , paths) ; }
};
}
