macro_rules! Kind {
    () => {
        # [doc = " Indicates whether the callsite is a span or event."] # [derive (Clone , Eq , PartialEq)] pub struct Kind (u8) ;
    };
}

Kind!()