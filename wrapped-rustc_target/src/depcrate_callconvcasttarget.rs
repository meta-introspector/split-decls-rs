// Generated macro for CastTarget (struct)
macro_rules! Depcrate_callconvCastTarget {
() => {
// Module: crate::callconv
// Provides: {"CastTarget"}
// Dependencies: {}
# [doc = " Describes the type used for `PassMode::Cast`."] # [doc = ""] # [doc = " Passing arguments in this mode works as follows: the registers in the `prefix` (the ones that"] # [doc = " are `Some`) get laid out one after the other (using `repr(C)` layout rules). Then the"] # [doc = " `rest.unit` register type gets repeated often enough to cover `rest.size`. This describes the"] # [doc = " actual type used for the call; the Rust type of the argument is then transmuted to this ABI type"] # [doc = " (and all data in the padding between the registers is dropped)."] # [derive (Clone , PartialEq , Eq , Hash , Debug , HashStable_Generic)] pub struct CastTarget { pub prefix : [Option < Reg > ; 8] , # [doc = " The offset of `rest` from the start of the value. Currently only implemented for a `Reg`"] # [doc = " pair created by the `offset_pair` method."] pub rest_offset : Option < Size > , pub rest : Uniform , pub attrs : ArgAttributes , }
};
}
