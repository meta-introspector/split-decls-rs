macro_rules! deps {
    () => {
        GetValue!();
        SortStrategy!();
        SortPairs!();
        Result!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < K : GetValue > SortStrategy for SortPairs < K > { fn try_add_pair (& mut self , val : & Value , key : & Value) -> Result < () > { SortPairs :: try_add_pair (self , val , key) } fn sort (& mut self) -> Vec < Value > { SortPairs :: sort (self) } }
    };
}

impl_146!()