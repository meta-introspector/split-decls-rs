macro_rules! SerializeId {
    () => {
        # [derive (Debug)] pub struct SerializeId < 'a > (& 'a Id) ;
    };
}

SerializeId!()