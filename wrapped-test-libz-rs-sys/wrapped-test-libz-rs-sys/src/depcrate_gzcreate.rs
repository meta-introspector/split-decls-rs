// Generated macro for create (function)
macro_rules! Depcrate_gzcreate {
() => {
// Module: crate::gz
// Provides: {"create"}
// Dependencies: {}
# [test] fn create () { let temp_dir_path = temp_base () ; let temp_dir = tempfile :: TempDir :: new_in (temp_dir_path) . unwrap () ; let temp_path = temp_dir . path () ; test_open ! (path (temp_path , "new.gz") , "w" , true) ; test_open ! (path (temp_path , "new.gz") , "wx" , false) ; test_open ! (path (temp_path , "different_file.gz") , "wx" , true) ; test_open ! (path (temp_path , "new.gz") , "ew" , true) ; test_open ! (path (temp_path , "new2.gz") , "a" , true) ; test_open ! (path (temp_path , "new2.gz") , "ax" , false) ; test_open ! (path (temp_path , "new3.gz") , "ax" , true) ; test_open ! (path (temp_path , "new4.gz") , "+" , false) ; }
};
}
