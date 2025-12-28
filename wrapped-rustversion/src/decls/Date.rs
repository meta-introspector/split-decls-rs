macro_rules! Date {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub struct Date { pub year : u16 , pub month : u8 , pub day : u8 , }
    };
}

Date!()