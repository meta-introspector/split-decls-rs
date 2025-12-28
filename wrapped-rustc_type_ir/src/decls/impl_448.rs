macro_rules! deps {
    () => {
        FloatVarValue!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        impl UnifyValue for FloatVarValue { type Error = NoError ; fn unify_values (value1 : & Self , value2 : & Self) -> Result < Self , Self :: Error > { match (* value1 , * value2) { (FloatVarValue :: Unknown , FloatVarValue :: Unknown) => Ok (FloatVarValue :: Unknown) , (FloatVarValue :: Unknown , FloatVarValue :: Known (known)) | (FloatVarValue :: Known (known) , FloatVarValue :: Unknown) => { Ok (FloatVarValue :: Known (known)) } (FloatVarValue :: Known (_) , FloatVarValue :: Known (_)) => { panic ! ("differing floats should have been resolved first") } } } }
    };
}

impl_448!();