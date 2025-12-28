macro_rules! member_message {
    () => {
        fn member_message (member : & Member) -> String { match member { Member :: Named (ident) => format ! ("`{}`" , ident) , Member :: Unnamed (i) => format ! ("#{}" , i . index) , } }
    };
}

member_message!();