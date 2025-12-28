macro_rules! deps {
    () => {
        TriColorVisitor!();
    };
}

macro_rules! CycleDetector {
    () => {
        deps!();
        # [doc = " This `TriColorVisitor` looks for back edges in a graph, which indicate that a cycle exists."] pub struct CycleDetector ;
    };
}

CycleDetector!();