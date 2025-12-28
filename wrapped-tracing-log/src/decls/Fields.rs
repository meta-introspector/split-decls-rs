macro_rules! Fields {
    () => {
        struct Fields { message : field :: Field , target : field :: Field , module : field :: Field , file : field :: Field , line : field :: Field , }
    };
}

Fields!();