// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
pub fn main () { print_env () ; println ! ("Call function sleep") ; sleep () ; println ! ("Call function fibonacci") ; let result = fibonacci (30) ; println ! ("fibonacci(30) = {result}") ; assert ! (result == 832040 , "Error in the calculation of fibonacci(30) ") ; for _i in 0 .. 10 { black_box (fibonacci (black_box (30))) ; } let now = Instant :: now () ; for _ in 0 .. N { black_box (fibonacci (black_box (30))) ; } let elapsed = now . elapsed () ; println ! ("Time to call {} times native_fibonacci(30): {} s" , N , elapsed . as_secs_f32 ()) ; for _i in 0 .. 10 { create_large_file () ; fs :: remove_file (FILE_PATH) . expect ("Could not delete file") ; } let start_time = Instant :: now () ; for _i in 0 .. M { create_large_file () ; fs :: remove_file (FILE_PATH) . expect ("Could not delete file") ; } let elapsed_time = start_time . elapsed () ; println ! ("Total Create File Time: {elapsed_time:?}") ; create_large_file () ; for _i in 0 .. 10 { read_large_file () ; } let start_time = Instant :: now () ; for _i in 0 .. M { read_large_file () ; } let elapsed_time = start_time . elapsed () ; println ! ("Total Read File Time: {elapsed_time:?}") ; fs :: remove_file (FILE_PATH) . expect ("Could not delete file") ; }
};
}
