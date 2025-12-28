macro_rules! deps {
    () => {
        Bridge!();
    };
}

macro_rules! CompilerCtxt {
    () => {
        deps!();
        # [doc = " Provides direct access to rustc's internal queries."] # [doc = ""] # [doc = " `CompilerInterface` must go through"] # [doc = " this context to obtain internal information."] pub struct CompilerCtxt < 'tcx , B : Bridge > { pub tcx : TyCtxt < 'tcx > , _marker : PhantomData < B > , }
    };
}

CompilerCtxt!()