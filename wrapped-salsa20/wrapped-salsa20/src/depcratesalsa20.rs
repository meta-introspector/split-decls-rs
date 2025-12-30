// Generated macro for Salsa20 (type)
macro_rules! DepcrateSalsa20 {
() => {
// Module: crate
// Provides: {"Salsa20"}
// Dependencies: {}
# [doc = " Salsa20/20 stream cipher"] # [doc = " (20 rounds; **recommended**)"] pub type Salsa20 = StreamCipherCoreWrapper < SalsaCore < U10 > > ;
};
}
