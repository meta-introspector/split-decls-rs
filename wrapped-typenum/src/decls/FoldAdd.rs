macro_rules! FoldAdd {
    () => {
        # [doc = " A **type operator** that gives the sum of all elements of an `Array`."] pub trait FoldAdd { # [doc = " The type of the result of the sum"] type Output ; }
    };
}

FoldAdd!()