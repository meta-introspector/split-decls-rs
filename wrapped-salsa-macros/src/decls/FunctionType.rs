macro_rules! deps {
    () => {
        SalsaStruct!();
    };
}

macro_rules! FunctionType {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Hash)] enum FunctionType { Constant , SalsaStruct , RequiresInterning , }
    };
}

FunctionType!();