macro_rules! deps {
    () => {
        AssocTypeData!();
        Symbol!();
    };
}

macro_rules! AssocKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AssocKind { Const { name : Symbol } , Fn { name : Symbol , has_self : bool } , Type { data : AssocTypeData } , }
    };
}

AssocKind!()