// Generated macro for Salsa12 (type)
macro_rules! DepcrateSalsa12 {
() => {
// Module: crate
// Provides: {"Salsa12"}
// Dependencies: {}
# [doc = " Salsa20/12 stream cipher"] # [doc = " (reduced-round variant of Salsa20 with 12 rounds, *not recommended*)"] pub type Salsa12 = StreamCipherCoreWrapper < SalsaCore < U6 > > ;
};
}
