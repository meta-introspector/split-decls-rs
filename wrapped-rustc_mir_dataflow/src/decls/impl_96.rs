macro_rules! deps {
    () => {
        AnalysisAndResults!();
        Analysis!();
        ResultsCursor!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 'tcx , A > AnalysisAndResults < 'tcx , A > where A : Analysis < 'tcx > , { # [doc = " Creates a `ResultsCursor` that takes ownership of `self`."] pub fn into_results_cursor < 'mir > (self , body : & 'mir Body < 'tcx >) -> ResultsCursor < 'mir , 'tcx , A > { ResultsCursor :: new_owning (body , self . analysis , self . results) } }
    };
}

impl_96!();