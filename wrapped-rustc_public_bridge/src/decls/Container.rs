macro_rules! deps {
    () => {
        Bridge!();
        Tables!();
        CompilerCtxt!();
    };
}

macro_rules! Container {
    () => {
        deps!();
        # [doc = " A container which is used for TLS."] pub struct Container < 'tcx , B : Bridge > { pub tables : RefCell < Tables < 'tcx , B > > , pub cx : RefCell < CompilerCtxt < 'tcx , B > > , }
    };
}

Container!();