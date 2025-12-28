macro_rules! deps {
    () => {
        EvalCtxt!();
        SolverDelegate!();
    };
}

macro_rules! ProbeCtxt {
    () => {
        deps!();
        pub (in crate :: solve) struct ProbeCtxt < 'me , 'a , D , I , F , T > where D : SolverDelegate < Interner = I > , I : Interner , { ecx : & 'me mut EvalCtxt < 'a , D , I > , probe_kind : F , _result : PhantomData < T > , }
    };
}

ProbeCtxt!()