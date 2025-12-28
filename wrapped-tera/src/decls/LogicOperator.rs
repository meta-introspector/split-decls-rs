macro_rules! LogicOperator {
    () => {
        # [doc = " All logic operators"] # [derive (Copy , Clone , Debug , PartialEq)] pub enum LogicOperator { # [doc = " >"] Gt , # [doc = " >="] Gte , # [doc = " <"] Lt , # [doc = " <="] Lte , # [doc = " =="] Eq , # [doc = " !="] NotEq , # [doc = " and"] And , # [doc = " or"] Or , }
    };
}

LogicOperator!();