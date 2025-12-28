macro_rules! deps {
    () => {
        QueryJob!();
        QueryStackFrame!();
    };
}

macro_rules! QueryJobInfo {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct QueryJobInfo < I > { pub query : QueryStackFrame < I > , pub job : QueryJob < I > , }
    };
}

QueryJobInfo!()