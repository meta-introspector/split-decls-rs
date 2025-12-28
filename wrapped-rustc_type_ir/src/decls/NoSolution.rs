macro_rules! NoSolution {
    () => {
        # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct NoSolution ;
    };
}

NoSolution!();