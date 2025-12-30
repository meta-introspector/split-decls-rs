// Generated macro for generate_stub_no_issue (function)
macro_rules! Depcrategenerate_stub_no_issue {
() => {
// Module: crate
// Provides: {"generate_stub_no_issue"}
// Dependencies: {}
fn generate_stub_no_issue (path : & Path , name : & str , description : & str) { let content = format ! (include_str ! ("stub-no-issue.md") , name = name , description = description) ; t ! (write (path , content) , path) ; }
};
}
