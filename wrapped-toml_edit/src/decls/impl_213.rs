macro_rules! deps {
    () => {
        ValueRepr!();
        Repr!();
        Decor!();
        Formatted!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl < T > Formatted < T > where T : ValueRepr , { # [doc = " Default-formatted value"] pub fn new (value : T) -> Self { Self { value , repr : None , decor : Default :: default () , } } pub (crate) fn set_repr_unchecked (& mut self , repr : Repr) { self . repr = Some (repr) ; } # [doc = " The wrapped value"] pub fn value (& self) -> & T { & self . value } # [doc = " The wrapped value"] pub fn into_value (self) -> T { self . value } # [doc = " Returns the raw representation, if available."] pub fn as_repr (& self) -> Option < & Repr > { self . repr . as_ref () } # [doc = " Returns the default raw representation."] # [cfg (feature = "display")] pub fn default_repr (& self) -> Repr { self . value . to_repr () } # [doc = " Returns a raw representation."] # [cfg (feature = "display")] pub fn display_repr (& self) -> Cow < '_ , str > { self . as_repr () . and_then (| r | r . as_raw () . as_str ()) . map (Cow :: Borrowed) . unwrap_or_else (| | { Cow :: Owned (self . default_repr () . as_raw () . as_str () . unwrap () . to_owned ()) }) } # [doc = " The location within the original document"] # [doc = ""] # [doc = " This generally requires a [`Document`][crate::Document]."] pub fn span (& self) -> Option < std :: ops :: Range < usize > > { self . repr . as_ref () . and_then (| r | r . span ()) } pub (crate) fn despan (& mut self , input : & str) { self . decor . despan (input) ; if let Some (repr) = & mut self . repr { repr . despan (input) ; } } # [doc = " Returns the surrounding whitespace"] pub fn decor_mut (& mut self) -> & mut Decor { & mut self . decor } # [doc = " Returns the surrounding whitespace"] pub fn decor (& self) -> & Decor { & self . decor } # [doc = " Auto formats the value."] pub fn fmt (& mut self) { self . repr = None ; } }
    };
}

impl_213!()