macro_rules! deps {
    () => {
        PatCx!();
        UsefulnessCtxt!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < 'a , 'p , Cx : PatCx > UsefulnessCtxt < 'a , 'p , Cx > { fn increase_complexity_level (& mut self , complexity_add : usize) -> Result < () , Cx :: Error > { self . complexity_level += complexity_add ; if self . complexity_level <= self . complexity_limit { Ok (()) } else { self . tycx . complexity_exceeded () } } }
    };
}

impl_91!()