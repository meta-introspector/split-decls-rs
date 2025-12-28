macro_rules! deps {
    () => {
        MaybeCause!();
        Certainty!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl Certainty { pub const AMBIGUOUS : Certainty = Certainty :: Maybe (MaybeCause :: Ambiguity) ; # [doc = " Use this function to merge the certainty of multiple nested subgoals."] # [doc = ""] # [doc = " Given an impl like `impl<T: Foo + Bar> Baz for T {}`, we have 2 nested"] # [doc = " subgoals whenever we use the impl as a candidate: `T: Foo` and `T: Bar`."] # [doc = " If evaluating `T: Foo` results in ambiguity and `T: Bar` results in"] # [doc = " success, we merge these two responses. This results in ambiguity."] # [doc = ""] # [doc = " If we unify ambiguity with overflow, we return overflow. This doesn't matter"] # [doc = " inside of the solver as we do not distinguish ambiguity from overflow. It does"] # [doc = " however matter for diagnostics. If `T: Foo` resulted in overflow and `T: Bar`"] # [doc = " in ambiguity without changing the inference state, we still want to tell the"] # [doc = " user that `T: Baz` results in overflow."] pub fn and (self , other : Certainty) -> Certainty { match (self , other) { (Certainty :: Yes , Certainty :: Yes) => Certainty :: Yes , (Certainty :: Yes , Certainty :: Maybe (_)) => other , (Certainty :: Maybe (_) , Certainty :: Yes) => self , (Certainty :: Maybe (a) , Certainty :: Maybe (b)) => Certainty :: Maybe (a . and (b)) , } } pub const fn overflow (suggest_increasing_limit : bool) -> Certainty { Certainty :: Maybe (MaybeCause :: Overflow { suggest_increasing_limit , keep_constraints : false }) } }
    };
}

impl_199!()