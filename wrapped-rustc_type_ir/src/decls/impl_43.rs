macro_rules! deps {
    () => {
        DefId!();
        SimplifiedType!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < DefId > SimplifiedType < DefId > { pub fn def (self) -> Option < DefId > { match self { SimplifiedType :: Adt (d) | SimplifiedType :: Foreign (d) | SimplifiedType :: Trait (d) | SimplifiedType :: Closure (d) | SimplifiedType :: Coroutine (d) | SimplifiedType :: CoroutineWitness (d) => Some (d) , _ => None , } } }
    };
}

impl_43!()