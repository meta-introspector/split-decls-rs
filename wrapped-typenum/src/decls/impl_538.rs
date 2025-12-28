macro_rules! deps {
    () => {
        FoldMul!();
        FoldProd!();
        Prod!();
        TArr!();
    };
}

macro_rules! impl_538 {
    () => {
        deps!();
        impl < V , A > FoldMul for TArr < V , A > where A : FoldMul , FoldProd < A > : Mul < V > , { type Output = Prod < FoldProd < A > , V > ; }
    };
}

impl_538!();