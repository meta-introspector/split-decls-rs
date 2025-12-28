macro_rules! SerializeField {
    () => {
        # [derive (Debug)] pub struct SerializeField < 'a > (& 'a Field) ;
    };
}

SerializeField!();