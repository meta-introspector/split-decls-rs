macro_rules! deps {
    () => {
        Dfa!();
        Def!();
        Tree!();
        Answer!();
        Reason!();
        Assume!();
    };
}

macro_rules! bool {
    () => {
        deps!();
        mod bool { use super :: * ; # [test] fn should_permit_identity_transmutation_tree () { let src = Tree :: bool () ; assert_eq ! (is_transmutable (& src , & src , Assume :: default ()) , Answer :: Yes) ; assert_eq ! (is_transmutable (& src , & src , Assume { validity : true , .. Assume :: default () }) , Answer :: Yes) ; } # [test] fn should_permit_identity_transmutation_dfa () { let src = Dfa :: bool () ; assert_eq ! (is_transmutable (& src , & src , Assume :: default ()) , Answer :: Yes) ; assert_eq ! (is_transmutable (& src , & src , Assume { validity : true , .. Assume :: default () }) , Answer :: Yes) ; } # [test] fn transmute_u8 () { let bool = & Tree :: bool () ; let u8 = & Tree :: u8 () ; for (src , dst , assume_validity , answer) in [(bool , u8 , false , Answer :: Yes) , (bool , u8 , true , Answer :: Yes) , (u8 , bool , false , Answer :: No (Reason :: DstIsBitIncompatible)) , (u8 , bool , true , Answer :: Yes) ,] { assert_eq ! (is_transmutable (src , dst , Assume { validity : assume_validity , .. Assume :: default () }) , answer) ; } } # [test] fn should_permit_validity_expansion_and_reject_contraction () { let b0 = layout :: Tree :: < Def , ! , ! > :: byte (0) ; let b1 = layout :: Tree :: < Def , ! , ! > :: byte (1) ; let b2 = layout :: Tree :: < Def , ! , ! > :: byte (2) ; let alts = [b0 , b1 , b2] ; let into_layout = | alts : Vec < _ > | { alts . into_iter () . fold (layout :: Tree :: < Def , ! , ! > :: uninhabited () , layout :: Tree :: < Def , ! , ! > :: or) } ; let into_set = | alts : Vec < _ > | { # [cfg (feature = "rustc")] let mut set = rustc_data_structures :: fx :: FxIndexSet :: default () ; # [cfg (not (feature = "rustc"))] let mut set = std :: collections :: HashSet :: new () ; set . extend (alts) ; set } ; for src_alts in alts . clone () . into_iter () . powerset () { let src_layout = into_layout (src_alts . clone ()) ; let src_set = into_set (src_alts . clone ()) ; for dst_alts in alts . clone () . into_iter () . powerset () . filter (| alts | ! alts . is_empty ()) { let dst_layout = into_layout (dst_alts . clone ()) ; let dst_set = into_set (dst_alts . clone ()) ; if src_set . is_subset (& dst_set) { assert_eq ! (Answer :: Yes , is_transmutable (& src_layout , & dst_layout , Assume :: default ()) , "{:?} SHOULD be transmutable into {:?}" , src_layout , dst_layout) ; } else if ! src_set . is_disjoint (& dst_set) { assert_eq ! (Answer :: Yes , is_transmutable (& src_layout , & dst_layout , Assume { validity : true , .. Assume :: default () }) , "{:?} SHOULD be transmutable (assuming validity) into {:?}" , src_layout , dst_layout) ; } else { assert_eq ! (Answer :: No (Reason :: DstIsBitIncompatible) , is_transmutable (& src_layout , & dst_layout , Assume :: default ()) , "{:?} should NOT be transmutable into {:?}" , src_layout , dst_layout) ; } } } } }
    };
}

bool!()