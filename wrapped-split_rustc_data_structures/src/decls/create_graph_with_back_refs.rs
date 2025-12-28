macro_rules! deps {
    () => {
        VecGraph!();
    };
}

macro_rules! create_graph_with_back_refs {
    () => {
        deps!();
        fn create_graph_with_back_refs () -> VecGraph < usize , true > { VecGraph :: new (7 , vec ! [(0 , 1) , (1 , 2) , (1 , 3) , (3 , 4) , (5 , 1)]) }
    };
}

create_graph_with_back_refs!();