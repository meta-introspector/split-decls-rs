macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! IterMutImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type IterMutImpl < 'a , K , V > = indexmap :: map :: IterMut < 'a , K , V > ;
    };
}

IterMutImpl!()