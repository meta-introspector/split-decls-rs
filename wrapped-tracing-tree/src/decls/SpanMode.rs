macro_rules! SpanMode {
    () => {
        # [derive (Debug , Copy , Clone)] pub (crate) enum SpanMode { # [doc = " Executed on the parent before entering a child span"] PreOpen , Open { verbose : bool , } , Close { verbose : bool , } , # [doc = " A span has been entered but another *different* span has been entered in the meantime."] Retrace { verbose : bool , } , PostClose , Event , }
    };
}

SpanMode!();