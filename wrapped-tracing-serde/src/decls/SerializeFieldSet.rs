macro_rules! SerializeFieldSet {
    () => {
        # [derive (Debug)] pub struct SerializeFieldSet < 'a > (& 'a FieldSet) ;
    };
}

SerializeFieldSet!()