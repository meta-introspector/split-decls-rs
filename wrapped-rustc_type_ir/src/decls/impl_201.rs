macro_rules! deps {
    () => {
        MaybeCause!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl MaybeCause { fn and (self , other : MaybeCause) -> MaybeCause { match (self , other) { (MaybeCause :: Ambiguity , MaybeCause :: Ambiguity) => MaybeCause :: Ambiguity , (MaybeCause :: Ambiguity , MaybeCause :: Overflow { .. }) => other , (MaybeCause :: Overflow { .. } , MaybeCause :: Ambiguity) => self , (MaybeCause :: Overflow { suggest_increasing_limit : limit_a , keep_constraints : keep_a , } , MaybeCause :: Overflow { suggest_increasing_limit : limit_b , keep_constraints : keep_b , } ,) => MaybeCause :: Overflow { suggest_increasing_limit : limit_a && limit_b , keep_constraints : keep_a && keep_b , } , } } pub fn or (self , other : MaybeCause) -> MaybeCause { match (self , other) { (MaybeCause :: Ambiguity , MaybeCause :: Ambiguity) => MaybeCause :: Ambiguity , (MaybeCause :: Ambiguity , MaybeCause :: Overflow { suggest_increasing_limit , keep_constraints : _ } ,) => MaybeCause :: Overflow { suggest_increasing_limit , keep_constraints : true } , (MaybeCause :: Overflow { suggest_increasing_limit , keep_constraints : _ } , MaybeCause :: Ambiguity ,) => MaybeCause :: Overflow { suggest_increasing_limit , keep_constraints : true } , (MaybeCause :: Overflow { suggest_increasing_limit : limit_a , keep_constraints : keep_a , } , MaybeCause :: Overflow { suggest_increasing_limit : limit_b , keep_constraints : keep_b , } ,) => MaybeCause :: Overflow { suggest_increasing_limit : limit_a || limit_b , keep_constraints : keep_a || keep_b , } , } } }
    };
}

impl_201!();