macro_rules! deps {
    () => {
        Dfa!();
        State!();
        Answer!();
        Assume!();
    };
}

macro_rules! union {
    () => {
        deps!();
        mod union { use super :: * ; # [test] fn union () { let [a , b , c , d] = [0 , 1 , 2 , 3] ; let s = Dfa :: from_edges (a , d , & [(a , 0 , b) , (b , 0 , d) , (a , 1 , c) , (c , 1 , d)]) ; let t = Dfa :: from_edges (a , c , & [(a , 1 , b) , (b , 0 , c)]) ; let mut ctr = 0 ; let new_state = | | { let state = crate :: layout :: dfa :: State (ctr) ; ctr += 1 ; state } ; let u = s . clone () . union (t . clone () , new_state) ; let expected_u = Dfa :: from_edges (b , a , & [(b , 0 ..= 0 , c) , (b , 1 ..= 1 , d) , (d , 0 ..= 1 , a) , (c , 0 ..= 0 , a)]) ; assert_eq ! (u , expected_u) ; assert_eq ! (is_transmutable (& s , & u , Assume :: default ()) , Answer :: Yes) ; assert_eq ! (is_transmutable (& t , & u , Assume :: default ()) , Answer :: Yes) ; } }
    };
}

union!();