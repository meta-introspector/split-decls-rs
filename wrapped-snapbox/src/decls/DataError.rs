macro_rules! deps {
    () => {
        Error!();
        DataFormat!();
    };
}

macro_rules! DataError {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct DataError { error : crate :: assert :: Error , intended : DataFormat , }
    };
}

DataError!();