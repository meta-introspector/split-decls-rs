macro_rules! deps {
    () => {
        Condition!();
        MaybeTransmutableQuery!();
        Answer!();
        Def!();
        Tree!();
        Assume!();
        Reference!();
    };
}

macro_rules! r#ref {
    () => {
        deps!();
        mod r#ref { use super :: * ; use crate :: layout :: Reference ; # [test] fn should_permit_identity_transmutation () { type Tree = crate :: layout :: Tree < Def , usize , () > ; for validity in [false , true] { let layout = Tree :: Seq (vec ! [Tree :: byte (0x00) , Tree :: Ref (Reference { region : 42 , is_mut : false , referent : () , referent_size : 0 , referent_align : 1 , }) ,]) ; let assume = Assume { validity , .. Assume :: default () } ; let answer = crate :: maybe_transmutable :: MaybeTransmutableQuery :: new (layout . clone () , layout , assume , UltraMinimal :: default () ,) . answer () ; assert_eq ! (answer , Answer :: If (Condition :: IfAll (vec ! [Condition :: Transmutable { src : () , dst : () } , Condition :: Outlives { long : 42 , short : 42 } , Condition :: Immutable { ty : () } ,]))) ; } } }
    };
}

r#ref!()