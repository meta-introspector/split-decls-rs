macro_rules! deps {
    () => {
        MultiName!();
        Default!();
    };
}

macro_rules! Field {
    () => {
        deps!();
        # [doc = " Represents field attribute information"] pub struct Field { name : MultiName , skip_serializing : bool , skip_deserializing : bool , skip_serializing_if : Option < syn :: ExprPath > , default : Default , serialize_with : Option < syn :: ExprPath > , deserialize_with : Option < syn :: ExprPath > , ser_bound : Option < Vec < syn :: WherePredicate > > , de_bound : Option < Vec < syn :: WherePredicate > > , borrowed_lifetimes : BTreeSet < syn :: Lifetime > , getter : Option < syn :: ExprPath > , flatten : bool , transparent : bool , }
    };
}

Field!();