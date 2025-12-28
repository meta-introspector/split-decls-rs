macro_rules! BlockType {
    () => {
        pub (crate) enum BlockType { StoredBlock = 0 , StaticTrees = 1 , DynamicTrees = 2 , }
    };
}

BlockType!()