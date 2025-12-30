// Generated macro for BUFFER_LEN (const)
macro_rules! Depcrate_scannerBUFFER_LEN {
() => {
// Module: crate::scanner
// Provides: {"BUFFER_LEN"}
// Dependencies: {}
# [doc = " The size of the [`Scanner`] buffer."] # [doc = ""] # [doc = " The buffer is statically allocated to avoid conditions for reallocations each time we"] # [doc = " consume/push a character. As of now, almost all lookaheads are 4 characters maximum, except:"] # [doc = "   - Escape sequences parsing: some escape codes are 8 characters"] # [doc = "   - Scanning indent in scalars: this looks ahead `indent + 2` characters"] # [doc = ""] # [doc = " This constant must be set to at least 8. When scanning indent in scalars, the lookahead is done"] # [doc = " in a single call if and only if the indent is `BUFFER_LEN - 2` or less. If the indent is higher"] # [doc = " than that, the code will fall back to a loop of lookaheads."] const BUFFER_LEN : usize = 16 ;
};
}
