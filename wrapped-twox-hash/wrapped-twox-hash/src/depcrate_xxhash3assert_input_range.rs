// Generated macro for assert_input_range (macro)
macro_rules! Depcrate_xxhash3assert_input_range {
() => {
// Module: crate::xxhash3
// Provides: {"assert_input_range"}
// Dependencies: {}
macro_rules ! assert_input_range { ($ min : literal .., $ len : expr) => { assert ! ($ min <= $ len) ; } ; ($ min : literal ..=$ max : literal , $ len : expr) => { assert ! ($ min <= $ len) ; assert ! ($ len <= $ max) ; } ; }
};
}
