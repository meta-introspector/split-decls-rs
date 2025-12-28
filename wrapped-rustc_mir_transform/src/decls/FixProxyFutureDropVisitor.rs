macro_rules! FixProxyFutureDropVisitor {
    () => {
        struct FixProxyFutureDropVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , replace_to : Local , }
    };
}

FixProxyFutureDropVisitor!();