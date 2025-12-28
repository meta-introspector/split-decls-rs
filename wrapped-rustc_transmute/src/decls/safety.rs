macro_rules! deps {
    () => {
        Answer!();
        Assume!();
        Def!();
        Reason!();
        Tree!();
    };
}

macro_rules! safety {
    () => {
        deps!();
        mod safety { use super :: * ; use crate :: Answer ; const DST_HAS_SAFETY_INVARIANTS : Answer < ! , ! > = Answer :: No (crate :: Reason :: DstMayHaveSafetyInvariants) ; # [test] fn src_safe_dst_safe () { let src = Tree :: Def (Def :: NoSafetyInvariants) . then (Tree :: u8 ()) ; let dst = Tree :: Def (Def :: NoSafetyInvariants) . then (Tree :: u8 ()) ; assert_eq ! (is_transmutable (& src , & dst , Assume :: default ()) , Answer :: Yes) ; assert_eq ! (is_transmutable (& src , & dst , Assume { safety : true , .. Assume :: default () }) , Answer :: Yes) ; } # [test] fn src_safe_dst_unsafe () { let src = Tree :: Def (Def :: NoSafetyInvariants) . then (Tree :: u8 ()) ; let dst = Tree :: Def (Def :: HasSafetyInvariants) . then (Tree :: u8 ()) ; assert_eq ! (is_transmutable (& src , & dst , Assume :: default ()) , DST_HAS_SAFETY_INVARIANTS) ; assert_eq ! (is_transmutable (& src , & dst , Assume { safety : true , .. Assume :: default () }) , Answer :: Yes) ; } # [test] fn src_unsafe_dst_safe () { let src = Tree :: Def (Def :: HasSafetyInvariants) . then (Tree :: u8 ()) ; let dst = Tree :: Def (Def :: NoSafetyInvariants) . then (Tree :: u8 ()) ; assert_eq ! (is_transmutable (& src , & dst , Assume :: default ()) , Answer :: Yes) ; assert_eq ! (is_transmutable (& src , & dst , Assume { safety : true , .. Assume :: default () }) , Answer :: Yes) ; } # [test] fn src_unsafe_dst_unsafe () { let src = Tree :: Def (Def :: HasSafetyInvariants) . then (Tree :: u8 ()) ; let dst = Tree :: Def (Def :: HasSafetyInvariants) . then (Tree :: u8 ()) ; assert_eq ! (is_transmutable (& src , & dst , Assume :: default ()) , DST_HAS_SAFETY_INVARIANTS) ; assert_eq ! (is_transmutable (& src , & dst , Assume { safety : true , .. Assume :: default () }) , Answer :: Yes) ; } }
    };
}

safety!()