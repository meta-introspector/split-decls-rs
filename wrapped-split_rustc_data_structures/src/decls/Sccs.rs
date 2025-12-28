macro_rules! deps {
    () => {
        SccData!();
    };
}

macro_rules! Sccs {
    () => {
        deps!();
        # [doc = " Strongly connected components (SCC) of a graph. The type `N` is"] # [doc = " the index type for the graph nodes and `S` is the index type for"] # [doc = " the SCCs. We can map from each node to the SCC that it"] # [doc = " participates in, and we also have the successors of each SCC."] pub struct Sccs < N : Idx , S : Idx > { # [doc = " For each node, what is the SCC index of the SCC to which it"] # [doc = " belongs."] scc_indices : IndexVec < N , S > , # [doc = " Data about all the SCCs."] scc_data : SccData < S > , }
    };
}

Sccs!()