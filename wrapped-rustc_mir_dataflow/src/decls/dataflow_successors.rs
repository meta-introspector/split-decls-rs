macro_rules! deps {
    () => {
        CfgEdge!();
    };
}

macro_rules! dataflow_successors {
    () => {
        deps!();
        fn dataflow_successors (body : & Body < '_ > , bb : BasicBlock) -> Vec < CfgEdge > { body [bb] . terminator () . successors () . enumerate () . map (| (index , _) | CfgEdge { source : bb , index }) . collect () }
    };
}

dataflow_successors!();