// Generated macro for TestInfo (struct)
macro_rules! DepcrateTestInfo {
() => {
// Module: crate
// Provides: {"TestInfo"}
// Dependencies: {}
# [doc = " Configuration for a single test."] # [derive (Debug)] pub struct TestInfo { pub name : String , float_name : & 'static str , float_bits : u32 , gen_name : & 'static str , # [doc = " Name for display in the progress bar."] short_name : String , # [doc = " Pad the short name to a common width for progress bar use."] short_name_padded : String , total_tests : u64 , # [doc = " Function to launch this test."] launch : fn (& TestInfo , & Config) , # [doc = " Progress bar to be updated."] progress : Option < ui :: Progress > , # [doc = " Once completed, this will be set."] completed : OnceLock < Completed > , }
};
}
