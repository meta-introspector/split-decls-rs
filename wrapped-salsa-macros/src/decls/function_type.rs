macro_rules! deps {
    () => {
        SalsaStruct!();
        FunctionType!();
    };
}

macro_rules! function_type {
    () => {
        deps!();
        fn function_type (item_fn : & syn :: ItemFn) -> FunctionType { match item_fn . sig . inputs . len () { 0 => unreachable ! ("functions have been checked to have at least a database argument by this point") , 1 => FunctionType :: Constant , 2 => FunctionType :: SalsaStruct , _ => FunctionType :: RequiresInterning , } }
    };
}

function_type!();