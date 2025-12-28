macro_rules! deps {
    () => {
        IntVarValue!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl UnifyValue for IntVarValue { type Error = NoError ; fn unify_values (value1 : & Self , value2 : & Self) -> Result < Self , Self :: Error > { match (* value1 , * value2) { (IntVarValue :: Unknown , IntVarValue :: Unknown) => Ok (IntVarValue :: Unknown) , (IntVarValue :: Unknown , known @ (IntVarValue :: UintType (_) | IntVarValue :: IntType (_)) ,) | (known @ (IntVarValue :: UintType (_) | IntVarValue :: IntType (_)) , IntVarValue :: Unknown ,) => Ok (known) , _ => panic ! ("differing ints should have been resolved first") , } } }
    };
}

impl_446!();