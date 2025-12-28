macro_rules! deps {
    () => {
        DataSourceInner!();
    };
}

macro_rules! DataSource {
    () => {
        deps!();
        # [doc = " Origin of a snapshot so it can be updated"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct DataSource { pub (crate) inner : DataSourceInner , }
    };
}

DataSource!();