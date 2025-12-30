// Generated macro for USAGE (const)
macro_rules! DepcrateUSAGE {
() => {
// Module: crate
// Provides: {"USAGE"}
// Dependencies: {}
const USAGE : & str = "\
usage:

cargo run -p util -- <SUBCOMMAND>

SUBCOMMAND:
    eval <BASIS> <OP> inputs...
        Evaulate the expression with a given basis. This can be useful for
        running routines with a debugger, or quickly checking input. Examples:
        * eval musl sinf 1.234 # print the results of musl sinf(1.234f32)
        * eval mpfr pow 1.234 2.432 # print the results of mpfr pow(1.234, 2.432)
" ;
};
}
