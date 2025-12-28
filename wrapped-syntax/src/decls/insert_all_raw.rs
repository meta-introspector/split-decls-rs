macro_rules! deps {
    () => {
        PositionRepr!();
        SyntaxElement!();
        Position!();
    };
}

macro_rules! insert_all_raw {
    () => {
        deps!();
        pub fn insert_all_raw (position : Position , elements : Vec < SyntaxElement >) { let (parent , index) = match position . repr { PositionRepr :: FirstChild (parent) => (parent , 0) , PositionRepr :: After (child) => (child . parent () . unwrap () , child . index () + 1) , } ; parent . splice_children (index .. index , elements) ; }
    };
}

insert_all_raw!()