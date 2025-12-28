macro_rules! deps {
    () => {
        Config!();
        Slab!();
    };
}

macro_rules! slab_eq {
    () => {
        deps!();
        # [track_caller] fn slab_eq (mut lhs : Slab < u64 , impl Config > , mut rhs : Slab < u64 , impl Config >) { let mut lhs_vec = lhs . unique_iter () . collect :: < Vec < _ > > () ; lhs_vec . sort_unstable () ; let mut rhs_vec = rhs . unique_iter () . collect :: < Vec < _ > > () ; rhs_vec . sort_unstable () ; assert_eq ! (lhs_vec , rhs_vec) ; }
    };
}

slab_eq!();