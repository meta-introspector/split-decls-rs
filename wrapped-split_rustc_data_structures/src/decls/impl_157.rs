macro_rules! deps {
    () => {
        Maxes!();
        MaxReached!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl Maxes { fn annotation (& self , scc : usize) -> MaxReached { self . 0 [scc] } fn new (mapping : fn (usize) -> usize) -> Self { Self (IndexVec :: new () , mapping) } }
    };
}

impl_157!()