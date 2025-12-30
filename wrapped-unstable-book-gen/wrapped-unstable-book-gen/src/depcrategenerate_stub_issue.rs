// Generated macro for generate_stub_issue (function)
macro_rules! Depcrategenerate_stub_issue {
() => {
// Module: crate
// Provides: {"generate_stub_issue"}
// Dependencies: {}
fn generate_stub_issue (path : & Path , name : & str , issue : u32 , description : & str) { let content = format ! (include_str ! ("stub-issue.md") , name = name , issue = issue , description = description) ; t ! (write (path , content) , path) ; }
};
}
