macro_rules! deps {
    () => {
        Env!();
        CaseSpec!();
        Bin!();
    };
}

macro_rules! RunnerSpec {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct RunnerSpec { cases : Vec < CaseSpec > , include : Option < Vec < String > > , default_bin : Option < crate :: schema :: Bin > , timeout : Option < std :: time :: Duration > , env : crate :: schema :: Env , }
    };
}

RunnerSpec!();