// Generated macro for test_cross (function)
macro_rules! Depcratetest_cross {
() => {
// Module: crate
// Provides: {"test_cross"}
// Dependencies: {}
fn test_cross (deny_warnings : bool) { println ! ("🧪 cross") ; let targets = ["thumbv6m-none-eabi" , "thumbv8m.base-none-eabi" , "riscv32i-unknown-none-elf" ,] ; let env = match deny_warnings { true => vec ! [("RUSTFLAGS" , "--deny warnings")] , false => vec ! [] , } ; let mut features = vec ! ["" , "alloc"] ; if ! rustc_is_msrv () { features . push ("ip_in_core") ; } for target in & targets { for feature in & features { do_test (| | { run_command ("cargo" , & ["check" , "--target" , target , "-p" , "defmt" , "--features" , feature] , None , & env ,) } , "cross" ,) ; do_test (| | { run_command ("cargo" , & ["check" , "--target" , target , "--features" , feature] , Some ("defmt-03") , & env ,) } , "cross-03" ,) ; } } do_test (| | { run_command ("cargo" , & ["check" , "--target" , "thumbv6m-none-eabi" , "--workspace" , "--exclude" , "defmt-itm" , "--exclude" , "firmware" ,] , Some ("firmware") , & env ,) } , "cross" ,) ; do_test (| | { run_command ("cargo" , & ["check" , "--target" , "thumbv7em-none-eabi"] , Some ("firmware") , & env ,) } , "cross" ,) ; for feature in ["print-defmt" , "print-rtt"] { do_test (| | { run_command ("cargo" , & ["check" , "--target" , "thumbv6m-none-eabi" , "--features" , feature] , Some ("firmware/panic-probe") , & env ,) } , "cross" ,) ; } }
};
}
