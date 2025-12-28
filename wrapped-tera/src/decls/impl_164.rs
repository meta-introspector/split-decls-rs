macro_rules! deps {
    () => {
        Result!();
        LogicOperator!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl fmt :: Display for LogicOperator { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , match * self { LogicOperator :: Gt => ">" , LogicOperator :: Gte => ">=" , LogicOperator :: Lt => "<" , LogicOperator :: Lte => "<=" , LogicOperator :: Eq => "==" , LogicOperator :: NotEq => "!=" , LogicOperator :: And => "and" , LogicOperator :: Or => "or" , }) } }
    };
}

impl_164!();