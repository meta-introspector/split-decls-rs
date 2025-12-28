macro_rules! deps {
    () => {
        CycleError!();
        QueryContext!();
        QueryInfo!();
        QueryStackFrameExtra!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < I > CycleError < I > { fn lift < Qcx : QueryContext < QueryInfo = I > > (& self , qcx : Qcx) -> CycleError < QueryStackFrameExtra > { CycleError { usage : self . usage . as_ref () . map (| (span , frame) | (* span , frame . lift (qcx))) , cycle : self . cycle . iter () . map (| info | info . lift (qcx)) . collect () , } } }
    };
}

impl_146!();