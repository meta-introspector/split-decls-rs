macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! ArrayOfTables {
    () => {
        deps!();
        # [doc = " A top-level sequence of [`Table`]s, each under their own header"] # [derive (Clone , Debug , Default)] pub struct ArrayOfTables { pub (crate) span : Option < std :: ops :: Range < usize > > , pub (crate) values : Vec < Item > , }
    };
}

ArrayOfTables!();