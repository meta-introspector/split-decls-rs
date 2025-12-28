macro_rules! deps {
    () => {
        MathOperator!();
        Result!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl fmt :: Display for MathOperator { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , match * self { MathOperator :: Add => "+" , MathOperator :: Sub => "-" , MathOperator :: Mul => "*" , MathOperator :: Div => "/" , MathOperator :: Modulo => "%" , }) } }
    };
}

impl_162!();