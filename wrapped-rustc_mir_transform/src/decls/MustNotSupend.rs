macro_rules! deps {
    () => {
        MustNotSuspendReason!();
    };
}

macro_rules! MustNotSupend {
    () => {
        deps!();
        pub (crate) struct MustNotSupend < 'a , 'tcx > { pub tcx : TyCtxt < 'tcx > , pub yield_sp : Span , pub reason : Option < MustNotSuspendReason > , pub src_sp : Span , pub pre : & 'a str , pub def_id : DefId , pub post : & 'a str , }
    };
}

MustNotSupend!()