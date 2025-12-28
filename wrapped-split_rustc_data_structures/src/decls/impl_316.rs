macro_rules! deps {
    () => {
        ForestObligation!();
        ProcessResult!();
        ClosureObligationProcessor!();
        TestOutcome!();
        ObligationProcessor!();
        Error!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl < OF , BF , O , E > ObligationProcessor for ClosureObligationProcessor < OF , BF , O , E > where O : super :: ForestObligation + fmt :: Debug , E : fmt :: Debug , OF : FnMut (& mut O) -> ProcessResult < O , E > , BF : FnMut (& [O]) , { type Obligation = O ; type Error = E ; type OUT = TestOutcome < O , E > ; fn needs_process_obligation (& self , _obligation : & Self :: Obligation) -> bool { true } fn process_obligation (& mut self , obligation : & mut Self :: Obligation ,) -> ProcessResult < Self :: Obligation , Self :: Error > { (self . process_obligation) (obligation) } fn process_backedge < 'c , I > (& mut self , _cycle : I , _marker : PhantomData < & 'c Self :: Obligation > ,) -> Result < () , Self :: Error > where I : Clone + Iterator < Item = & 'c Self :: Obligation > , { Ok (()) } }
    };
}

impl_316!();