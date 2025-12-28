macro_rules! deps {
    () => {
        QueryStackFrameExtra!();
        QueryStackFrame!();
        QueryInfo!();
    };
}

macro_rules! CycleError {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct CycleError < I = QueryStackFrameExtra > { # [doc = " The query and related span that uses the cycle."] pub usage : Option < (Span , QueryStackFrame < I >) > , pub cycle : Vec < QueryInfo < I > > , }
    };
}

CycleError!();