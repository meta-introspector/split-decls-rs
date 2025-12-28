macro_rules! SerializeFieldMap {
    () => {
        # [derive (Debug)] pub struct SerializeFieldMap < 'a , T > (& 'a T) ;
    };
}

SerializeFieldMap!();