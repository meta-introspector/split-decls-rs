macro_rules! DataType {
    () => {
        # [derive (Debug , PartialEq , Eq)] enum DataType { Binary = 0 , Text = 1 , Unknown = 2 , }
    };
}

DataType!();