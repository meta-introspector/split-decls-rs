macro_rules! deps {
    () => {
        AssertLinear!();
        Round!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl AssertLinear { pub fn next_round (& mut self) -> bool { if let Some (round) = self . rounds . last_mut () { round . finish () ; } if self . rounds . iter () . any (| it | it . linear) || self . rounds . len () == 4 { return false ; } self . rounds . push (Round :: default ()) ; true } pub fn sample (& mut self , x : f64 , y : f64) { self . rounds . last_mut () . unwrap () . samples . push ((x , y)) ; } }
    };
}

impl_2!();