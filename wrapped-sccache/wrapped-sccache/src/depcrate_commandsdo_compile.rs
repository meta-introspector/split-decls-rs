// Generated macro for do_compile (function)
macro_rules! Depcrate_commandsdo_compile {
() => {
// Module: crate::commands
// Provides: {"do_compile"}
// Dependencies: {}
# [doc = " Send a `Compile` request to the sccache server `conn`, and handle the response."] # [doc = ""] # [doc = " The first entry in `cmdline` will be looked up in `path` if it is not"] # [doc = " an absolute path."] # [doc = " See `request_compile` and `handle_compile_response`."] # [allow (clippy :: too_many_arguments)] pub fn do_compile < T > (creator : T , runtime : & mut Runtime , mut conn : ServerConnection , exe : & Path , cmdline : Vec < OsString > , cwd : & Path , path : Option < OsString > , env_vars : Vec < (OsString , OsString) > , stdout : & mut dyn Write , stderr : & mut dyn Write ,) -> Result < i32 > where T : CommandCreatorSync , { trace ! ("do_compile") ; let exe_path = which_in (exe , path , cwd) ? ; let res = request_compile (& mut conn , & exe_path , & cmdline , cwd , env_vars) ? ; handle_compile_response (creator , runtime , & mut conn , res , & exe_path , cmdline , cwd , stdout , stderr ,) }
};
}
