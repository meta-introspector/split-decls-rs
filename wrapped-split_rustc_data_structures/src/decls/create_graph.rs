macro_rules! deps {
    () => {
        VecGraph!();
    };
}

macro_rules! create_graph {
    () => {
        deps!();
        fn create_graph () -> VecGraph < usize > { VecGraph :: new (7 , vec ! [(0 , 1) , (1 , 2) , (1 , 3) , (3 , 4) , (5 , 1)]) }
    };
}

create_graph!()