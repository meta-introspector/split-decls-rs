macro_rules! ClosureObligationProcessor {
    () => {
        struct ClosureObligationProcessor < OF , BF , O , E > { process_obligation : OF , _process_backedge : BF , marker : PhantomData < (O , E) > , }
    };
}

ClosureObligationProcessor!()