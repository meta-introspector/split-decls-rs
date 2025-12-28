macro_rules! deps {
    () => {
        MaxReached!();
        Annotation!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl Annotation for MaxReached { fn merge_scc (self , other : Self) -> Self { Self (std :: cmp :: max (other . 0 , self . 0)) } fn merge_reached (self , other : Self) -> Self { Self (std :: cmp :: max (other . 0 , self . 0)) } }
    };
}

impl_158!()