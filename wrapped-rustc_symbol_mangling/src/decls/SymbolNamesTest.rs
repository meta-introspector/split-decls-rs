macro_rules! SymbolNamesTest {
    () => {
        struct SymbolNamesTest < 'tcx > { tcx : TyCtxt < 'tcx > , }
    };
}

SymbolNamesTest!();