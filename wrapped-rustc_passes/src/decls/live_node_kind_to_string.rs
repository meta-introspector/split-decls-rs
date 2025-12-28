macro_rules! deps {
    () => {
        LiveNodeKind!();
    };
}

macro_rules! live_node_kind_to_string {
    () => {
        deps!();
        fn live_node_kind_to_string (lnk : LiveNodeKind , tcx : TyCtxt < '_ >) -> String { let sm = tcx . sess . source_map () ; match lnk { UpvarNode (s) => format ! ("Upvar node [{}]" , sm . span_to_diagnostic_string (s)) , ExprNode (s , _) => format ! ("Expr node [{}]" , sm . span_to_diagnostic_string (s)) , VarDefNode (s , _) => format ! ("Var def node [{}]" , sm . span_to_diagnostic_string (s)) , ClosureNode => "Closure node" . to_owned () , ExitNode => "Exit node" . to_owned () , } }
    };
}

live_node_kind_to_string!();