// Generated macro for smoke_test_encoding (function)
macro_rules! Depcratesmoke_test_encoding {
() => {
// Module: crate
// Provides: {"smoke_test_encoding"}
// Dependencies: {}
# [test] fn smoke_test_encoding () { let encoding = if cfg ! (target_pointer_width = "64") { "@16@0:8" } else { "@8@0:4" } ; check_method :: < () , * mut NSObject > (NSObject :: class () , sel ! (self) , encoding) ; }
};
}
