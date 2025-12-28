macro_rules! DEP_NODE_SIZE {
    () => {
        const DEP_NODE_SIZE : usize = size_of :: < SerializedDepNodeIndex > () ;
    };
}

DEP_NODE_SIZE!()