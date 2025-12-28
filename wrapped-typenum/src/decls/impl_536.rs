macro_rules! deps {
    () => {
        FoldSum!();
        TArr!();
        Sum!();
        FoldAdd!();
    };
}

macro_rules! impl_536 {
    () => {
        deps!();
        impl < V , A > FoldAdd for TArr < V , A > where A : FoldAdd , FoldSum < A > : Add < V > , { type Output = Sum < FoldSum < A > , V > ; }
    };
}

impl_536!();