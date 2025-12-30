// Generated macro for Salsa8 (type)
macro_rules! DepcrateSalsa8 {
() => {
// Module: crate
// Provides: {"Salsa8"}
// Dependencies: {}
# [doc = " Salsa20/8 stream cipher"] # [doc = " (reduced-round variant of Salsa20 with 8 rounds, *not recommended*)"] pub type Salsa8 = StreamCipherCoreWrapper < SalsaCore < U4 > > ;
};
}
