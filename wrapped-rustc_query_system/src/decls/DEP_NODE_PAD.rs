macro_rules! DEP_NODE_PAD {
    () => {
        # [doc = " Amount of padding we need to add to the edge list data so that we can retrieve every"] # [doc = " SerializedDepNodeIndex with a fixed-size read then mask."] const DEP_NODE_PAD : usize = DEP_NODE_SIZE - 1 ;
    };
}

DEP_NODE_PAD!()