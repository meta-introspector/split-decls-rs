// Generated macro for Token (trait)
macro_rules! DepcrateToken {
() => {
// Module: crate
// Provides: {"Token"}
// Dependencies: {}
# [doc = " Trait for types that represent single tokens, such as keywords"] # [doc = " and operators."] # [decl (trait , name = "Token" , vis = "pub" , hash = "2cec615e")] pub trait Token where Self : Copy + Sized , { fn token (& self) -> & 'static str ; }
};
}
