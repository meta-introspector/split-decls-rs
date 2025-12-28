macro_rules! macro_82 {
    () => {
        pin_project ! { # [doc = " A future that may have completed."] # [derive (Debug)] # [project = MaybeDoneProj] # [project_replace = MaybeDoneProjReplace] # [repr (C)] pub enum MaybeDone < Fut : Future > { # [doc = " A not-yet-completed future."] Future { # [pin] future : Fut } , # [doc = " The output of the completed future."] Done { output : Fut :: Output } , # [doc = " The empty variant after the result of a [`MaybeDone`] has been"] # [doc = " taken using the [`take_output`](MaybeDone::take_output) method."] Gone , } }
    };
}

macro_82!()