macro_rules! deps {
    () => {
        VecCache!();
    };
}

macro_rules! impl_682 {
    () => {
        deps!();
        impl < K : Idx , V , I > Default for VecCache < K , V , I > { fn default () -> Self { VecCache { buckets : Default :: default () , key : PhantomData , len : Default :: default () , present : Default :: default () , } } }
    };
}

impl_682!();