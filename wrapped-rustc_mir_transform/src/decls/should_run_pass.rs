macro_rules! deps {
    () => {
        MirPass!();
        Optimizations!();
    };
}

macro_rules! should_run_pass {
    () => {
        deps!();
        pub (super) fn should_run_pass < 'tcx , P > (tcx : TyCtxt < 'tcx > , pass : & P , optimizations : Optimizations ,) -> bool where P : MirPass < 'tcx > + ? Sized , { let name = pass . name () ; if ! pass . can_be_overridden () { return pass . is_enabled (tcx . sess) ; } let overridden_passes = & tcx . sess . opts . unstable_opts . mir_enable_passes ; let overridden = overridden_passes . iter () . rev () . find (| (s , _) | s == & * name) . map (| (_name , polarity) | { trace ! (pass = % name , "{} as requested by flag" , if * polarity { "Running" } else { "Not running" } ,) ; * polarity }) ; let suppressed = ! pass . is_required () && matches ! (optimizations , Optimizations :: Suppressed) ; overridden . unwrap_or_else (| | ! suppressed && pass . is_enabled (tcx . sess)) }
    };
}

should_run_pass!();