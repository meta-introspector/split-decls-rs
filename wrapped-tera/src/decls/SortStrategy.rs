macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! SortStrategy {
    () => {
        deps!();
        pub trait SortStrategy { fn try_add_pair (& mut self , val : & Value , key : & Value) -> Result < () > ; fn sort (& mut self) -> Vec < Value > ; }
    };
}

SortStrategy!()