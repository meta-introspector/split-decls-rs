macro_rules! LPHANDLER_FUNCTION_EX {
    () => {
        pub type LPHANDLER_FUNCTION_EX = Option < unsafe extern "system" fn (dwcontrol : u32 , dweventtype : u32 , lpeventdata : * mut core :: ffi :: c_void , lpcontext : * mut core :: ffi :: c_void ,) -> u32 , > ;
    };
}

LPHANDLER_FUNCTION_EX!()