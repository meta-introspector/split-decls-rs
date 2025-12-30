// Generated macro for HexOptions (struct)
macro_rules! DepcrateHexOptions {
() => {
// Module: crate
// Provides: {"HexOptions"}
// Dependencies: {}
# [doc = " HEADS UP this is currently an implementation detail and not subject to semver guarantees."] # [doc = " do NOT use this outside the `ufmt` crate"] # [doc (hidden)] pub struct HexOptions { # [doc = " when we need to use digits a-f, should they be upper case instead?"] pub upper_case : bool , # [doc = " when we are padding to a target length, what character should we pad using?"] pub pad_char : u8 , # [doc = " when we are padding to a target length, how long should our string be?"] pub pad_length : usize , # [doc = " should we include a 0x prefix? (also controlled by upper_case)"] pub ox_prefix : bool , }
};
}
