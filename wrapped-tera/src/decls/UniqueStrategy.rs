macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! UniqueStrategy {
    () => {
        deps!();
        pub trait UniqueStrategy { fn insert (& mut self , val : & Value) -> Result < bool > ; }
    };
}

UniqueStrategy!();