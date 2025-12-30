// Generated macro for CheckFailure (enum)
macro_rules! DepcrateCheckFailure {
() => {
// Module: crate
// Provides: {"CheckFailure"}
// Dependencies: {}
# [doc = " Result of an input did not parsing successfully."] # [derive (Clone , Debug)] enum CheckFailure { # [doc = " Above the zero cutoff but got rounded to zero."] UnexpectedZero , # [doc = " Below the infinity cutoff but got rounded to infinity."] UnexpectedInf , # [doc = " Above the negative infinity cutoff but got rounded to negative infinity."] UnexpectedNegInf , # [doc = " Got a `NaN` when none was expected."] UnexpectedNan , # [doc = " Expected `NaN`, got none."] ExpectedNan , # [doc = " Expected infinity, got finite."] ExpectedInf , # [doc = " Expected negative infinity, got finite."] ExpectedNegInf , # [doc = " The value exceeded its error tolerance."] InvalidReal { # [doc = " Error from the expected value, as a float."] error_float : Option < f64 > , # [doc = " Error as a rational string (since it can't always be represented as a float)."] error_str : Box < str > , # [doc = " True if the error was caused by not rounding to even at the midpoint between"] # [doc = " two representable values."] incorrect_midpoint_rounding : bool , } , # [doc = " String did not parse successfully."] ParsingFailed (Box < str >) , # [doc = " A panic was caught."] Panic (Box < str >) , }
};
}
