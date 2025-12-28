macro_rules! deps {
    () => {
        DeArray!();
        DeValue!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < 'i > DeArray < 'i > { # [doc = " Constructs a new, empty `DeArray`."] # [doc = ""] # [doc = " This will not allocate until elements are pushed onto it."] pub const fn new () -> Self { Self { items : Vec :: new () , array_of_tables : false , } } # [doc = " Appends an element to the back of a collection."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the new capacity exceeds `isize::MAX` _bytes_."] pub fn push (& mut self , value : Spanned < DeValue < 'i > >) { self . items . push (value) ; } }
    };
}

impl_181!()