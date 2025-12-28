macro_rules! deps {
    () => {
        Def!();
        Region!();
        Type!();
    };
}

macro_rules! QueryContext {
    () => {
        deps!();
        # [doc = " Context necessary to answer the question \"Are these types transmutable?\"."] pub (crate) trait QueryContext { type Def : layout :: Def ; type Region : layout :: Region ; type Type : layout :: Type ; }
    };
}

QueryContext!()