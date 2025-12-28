macro_rules! FoldMul {
    () => {
        # [doc = " A **type operator** that gives the product of all elements of an `Array`."] pub trait FoldMul { # [doc = " The type of the result of the product"] type Output ; }
    };
}

FoldMul!()