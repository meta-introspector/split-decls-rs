// Generated macro for impl_1157 (impl)
macro_rules! Depcrate_distimpl_1157 {
() => {
// Module: crate::dist
// Provides: {"impl_1157"}
// Dependencies: {}
impl From < ProcessOutput > for process :: Output { fn from (o : ProcessOutput) -> Self { process :: Output { status : exit_status (o . code) , stdout : o . stdout , stderr : o . stderr , } } }
};
}
