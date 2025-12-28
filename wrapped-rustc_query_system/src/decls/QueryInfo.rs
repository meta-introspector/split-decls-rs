macro_rules! deps {
    () => {
        QueryStackFrame!();
    };
}

macro_rules! QueryInfo {
    () => {
        deps!();
        # [doc = " Represents a span and a query key."] # [derive (Clone , Debug)] pub struct QueryInfo < I > { # [doc = " The span corresponding to the reason for which this query was required."] pub span : Span , pub query : QueryStackFrame < I > , }
    };
}

QueryInfo!();