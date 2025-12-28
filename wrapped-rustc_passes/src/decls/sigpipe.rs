macro_rules! sigpipe {
    () => {
        fn sigpipe (tcx : TyCtxt < '_ >) -> u8 { match tcx . sess . opts . unstable_opts . on_broken_pipe { rustc_target :: spec :: OnBrokenPipe :: Default => sigpipe :: DEFAULT , rustc_target :: spec :: OnBrokenPipe :: Kill => sigpipe :: SIG_DFL , rustc_target :: spec :: OnBrokenPipe :: Error => sigpipe :: SIG_IGN , rustc_target :: spec :: OnBrokenPipe :: Inherit => sigpipe :: INHERIT , } }
    };
}

sigpipe!()