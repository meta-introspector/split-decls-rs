macro_rules! MathOperator {
    () => {
        # [doc = " All math operators"] # [derive (Copy , Clone , Debug , PartialEq)] pub enum MathOperator { # [doc = " +"] Add , # [doc = " -"] Sub , # [doc = " *"] Mul , # [doc = " /"] Div , # [doc = " %"] Modulo , }
    };
}

MathOperator!()