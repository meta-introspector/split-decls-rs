// Generated macro for TEST_PROVIDERS (const)
macro_rules! DepcrateTEST_PROVIDERS {
() => {
// Module: crate
// Provides: {"TEST_PROVIDERS"}
// Dependencies: {}
# [cfg (test)] const TEST_PROVIDERS : & [& crypto :: CryptoProvider] = & [# [cfg (feature = "aws-lc-rs")] & crypto :: aws_lc_rs :: DEFAULT_PROVIDER , # [cfg (feature = "ring")] & crypto :: ring :: DEFAULT_PROVIDER ,] ;
};
}
