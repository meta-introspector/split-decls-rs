macro_rules! deps {
    () => {
        MirPass!();
        WithMinOptLevel!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'tcx , T > MirPass < 'tcx > for WithMinOptLevel < T > where T : MirPass < 'tcx > , { fn name (& self) -> & 'static str { self . 1 . name () } fn is_enabled (& self , sess : & Session) -> bool { sess . mir_opt_level () >= self . 0 as usize } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { self . 1 . run_pass (tcx , body) } fn is_required (& self) -> bool { self . 1 . is_required () } }
    };
}

impl_8!()