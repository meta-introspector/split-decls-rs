// Generated macro for test_server_unsupported_compiler (function)
macro_rules! Depcrate_test_teststest_server_unsupported_compiler {
() => {
// Module: crate::test::tests
// Provides: {"test_server_unsupported_compiler"}
// Dependencies: {}
# [test] fn test_server_unsupported_compiler () { let f = TestFixture :: new () ; let (addr , sender , server_creator , child) = run_server_thread (f . tempdir . path () , None) ; let conn = connect_to_server (& addr) . unwrap () ; { let mut c = server_creator . lock () . unwrap () ; c . next_command_spawns (Ok (MockChild :: new (exit_status (1) , "hello" , "error"))) ; c . next_command_spawns (Ok (MockChild :: new (exit_status (0) , "hello" , "error"))) ; } let exe = & f . bins [0] ; let cmdline = vec ! ["-c" . into () , "file.c" . into () , "-o" . into () , "file.o" . into ()] ; let cwd = f . tempdir . path () ; let client_creator = new_creator () ; let mut stdout = Cursor :: new (Vec :: new ()) ; let mut stderr = Cursor :: new (Vec :: new ()) ; let path = Some (f . paths) ; let mut runtime = Runtime :: new () . unwrap () ; let res = do_compile (client_creator , & mut runtime , conn , exe , cmdline , cwd , path , vec ! [] , & mut stdout , & mut stderr ,) ; match res { Ok (_) => panic ! ("do_compile should have failed!") , Err (e) => assert_eq ! ("Compiler not supported: \"error\"" , e . to_string ()) , } assert_eq ! (0 , server_creator . lock () . unwrap () . children . len ()) ; sender . send (ServerMessage :: Shutdown) . ok () . unwrap () ; child . join () . unwrap () ; }
};
}
