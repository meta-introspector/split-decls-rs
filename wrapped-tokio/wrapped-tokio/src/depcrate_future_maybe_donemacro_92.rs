// Generated macro for macro_92 (macro)
macro_rules! Depcrate_future_maybe_donemacro_92 {
() => {
// Module: crate::future::maybe_done
// Provides: {"macro_92"}
// Dependencies: {}
pin_project ! { # [doc = " A future that may have completed."] # [derive (Debug)] # [project = MaybeDoneProj] # [project_replace = MaybeDoneProjReplace] # [repr (C)] pub enum MaybeDone < Fut : Future > { # [doc = " A not-yet-completed future."] Future { # [pin] future : Fut } , # [doc = " The output of the completed future."] Done { output : Fut :: Output } , # [doc = " The empty variant after the result of a [`MaybeDone`] has been"] # [doc = " taken using the [`take_output`](MaybeDone::take_output) method."] Gone , } }
};
}
