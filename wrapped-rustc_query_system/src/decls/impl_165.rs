macro_rules! deps {
    () => {
        QueryInfo!();
        QueryStackFrameExtra!();
        QueryContext!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < I > QueryInfo < I > { pub (crate) fn lift < Qcx : QueryContext < QueryInfo = I > > (& self , qcx : Qcx ,) -> QueryInfo < QueryStackFrameExtra > { QueryInfo { span : self . span , query : self . query . lift (qcx) } } }
    };
}

impl_165!();