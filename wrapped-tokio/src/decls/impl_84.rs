macro_rules! impl_84 {
    () => {
        impl < Fut : Future > MaybeDone < Fut > { # [doc = " Returns an [`Option`] containing a mutable reference to the output of the future."] # [doc = " The output of this method will be [`Some`] if and only if the inner"] # [doc = " future has been completed and [`take_output`](MaybeDone::take_output)"] # [doc = " has not yet been called."] pub fn output_mut (self : Pin < & mut Self >) -> Option < & mut Fut :: Output > { match self . project () { MaybeDoneProj :: Done { output } => Some (output) , _ => None , } } # [doc = " Attempts to take the output of a `MaybeDone` without driving it"] # [doc = " towards completion."] # [inline] pub fn take_output (self : Pin < & mut Self >) -> Option < Fut :: Output > { match * self { MaybeDone :: Done { .. } => { } MaybeDone :: Future { .. } | MaybeDone :: Gone => return None , } ; if let MaybeDoneProjReplace :: Done { output } = self . project_replace (MaybeDone :: Gone) { Some (output) } else { unreachable ! () } } }
    };
}

impl_84!()