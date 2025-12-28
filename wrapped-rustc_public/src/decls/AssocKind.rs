macro_rules! deps {
    () => {
        Symbol!();
        AssocTypeData!();
    };
}

macro_rules! AssocKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AssocKind { Const { name : Symbol } , Fn { name : Symbol , has_self : bool } , Type { data : AssocTypeData } , }
    };
}

AssocKind!();