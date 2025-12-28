macro_rules! deps {
    () => {
        Dfa!();
        Assume!();
        Tree!();
        Answer!();
        MaybeTransmutableQuery!();
    };
}

macro_rules! benches {
    () => {
        deps!();
        mod benches { use std :: hint :: black_box ; use test :: Bencher ; use super :: * ; # [bench] fn bench_dfa_from_tree (b : & mut Bencher) { let num = Tree :: number (8) . prune (& | _ | false) ; let num = black_box (num) ; b . iter (| | { let _ = black_box (Dfa :: from_tree (num . clone ())) ; }) } # [bench] fn bench_transmute (b : & mut Bencher) { let num = Tree :: number (8) . prune (& | _ | false) ; let dfa = black_box (Dfa :: from_tree (num) . unwrap ()) ; b . iter (| | { let answer = crate :: maybe_transmutable :: MaybeTransmutableQuery :: new (dfa . clone () , dfa . clone () , Assume :: default () , UltraMinimal :: default () ,) . answer () ; let answer = std :: hint :: black_box (answer) ; assert_eq ! (answer , Answer :: Yes) ; }) } }
    };
}

benches!()