macro_rules! deps {
    () => {
        Tree!();
    };
}

macro_rules! random_tree {
    () => {
        deps!();
        fn random_tree (depth : usize) -> Tree < u32 > { assert ! (depth > 0) ; let mut seed = < XorShiftRng as SeedableRng > :: Seed :: default () ; (0 ..) . zip (seed . as_mut ()) . for_each (| (i , x) | * x = i) ; let mut rng = XorShiftRng :: from_seed (seed) ; random_tree1 (depth , & mut rng) }
    };
}

random_tree!()