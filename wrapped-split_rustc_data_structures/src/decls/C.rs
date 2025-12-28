macro_rules! deps {
    () => {
        ClosureObligationProcessor!();
        ProcessResult!();
    };
}

macro_rules! C {
    () => {
        deps!();
        # [allow (non_snake_case)] fn C < OF , BF , O > (of : OF , bf : BF) -> ClosureObligationProcessor < OF , BF , O , & 'static str > where OF : FnMut (& mut O) -> ProcessResult < O , & 'static str > , BF : FnMut (& [O]) , { ClosureObligationProcessor { process_obligation : of , _process_backedge : bf , marker : PhantomData , } }
    };
}

C!()