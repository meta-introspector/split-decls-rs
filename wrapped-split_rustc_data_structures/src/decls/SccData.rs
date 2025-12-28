macro_rules! deps {
    () => {
        SccDetails!();
        Sccs!();
    };
}

macro_rules! SccData {
    () => {
        deps!();
        struct SccData < S : Idx > { # [doc = " Maps SCC indices to their metadata, including"] # [doc = " offsets into `all_successors`."] scc_details : IndexVec < S , SccDetails > , # [doc = " Contains the successors for all the Sccs, concatenated. The"] # [doc = " range of indices corresponding to a given SCC is found in its"] # [doc = " `scc_details.range`."] all_successors : Vec < S > , }
    };
}

SccData!();