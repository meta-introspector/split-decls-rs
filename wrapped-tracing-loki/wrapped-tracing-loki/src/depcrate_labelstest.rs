// Generated macro for test (module)
macro_rules! Depcrate_labelstest {
() => {
// Module: crate::labels
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: FormattedLabels ; use tracing_core :: Level ; # [test] fn simple () { assert_eq ! (FormattedLabels :: new () . finish (Level :: TRACE) , r#"{level="trace"}"# ,) ; assert_eq ! (FormattedLabels :: new () . finish (Level :: DEBUG) , r#"{level="debug"}"# ,) ; assert_eq ! (FormattedLabels :: new () . finish (Level :: INFO) , r#"{level="info"}"# ,) ; assert_eq ! (FormattedLabels :: new () . finish (Level :: WARN) , r#"{level="warn"}"# ,) ; assert_eq ! (FormattedLabels :: new () . finish (Level :: ERROR) , r#"{level="error"}"# ,) ; } # [test] fn level () { assert ! (FormattedLabels :: new () . add ("level" . into () , "") . is_err ()) ; assert ! (FormattedLabels :: new () . add ("level" . into () , "blurb") . is_err ()) ; } # [test] fn duplicate () { let mut labels = FormattedLabels :: new () ; labels . add ("label" . into () , "abc") . unwrap () ; assert ! (labels . clone () . add ("label" . into () , "def") . is_err ()) ; assert ! (labels . clone () . add ("label" . into () , "abc") . is_err ()) ; assert ! (labels . clone () . add ("label" . into () , "") . is_err ()) ; } }
};
}
