macro_rules! deps {
    () => {
        Scalar!();
    };
}

macro_rules! ValueAbi {
    () => {
        deps!();
        # [doc = " Describes how values of the type are passed by target ABIs,"] # [doc = " in terms of categories of C types there are ABI rules for."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub enum ValueAbi { Scalar (Scalar) , ScalarPair (Scalar , Scalar) , Vector { element : Scalar , count : u64 , } , Aggregate { # [doc = " If true, the size is exact, otherwise it's only a lower bound."] sized : bool , } , }
    };
}

ValueAbi!()