macro_rules! deps {
    () => {
        Lint!();
        MirPass!();
    };
}

macro_rules! MirLint {
    () => {
        deps!();
        # [doc = " Just like `MirPass`, except it cannot mutate `Body`, and MIR dumping is"] # [doc = " disabled (via the `Lint` adapter)."] pub (super) trait MirLint < 'tcx > { fn name (& self) -> & 'static str { const { simplify_pass_type_name (std :: any :: type_name :: < Self > ()) } } fn is_enabled (& self , _sess : & Session) -> bool { true } fn run_lint (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) ; }
    };
}

MirLint!();