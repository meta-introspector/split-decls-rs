macro_rules! deps {
    () => {
        Identifier!();
        Event!();
        Kind!();
        FieldSet!();
        Metadata!();
        Level!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < 'a > Metadata < 'a > { # [doc = " Construct new metadata for a span or event, with a name, target, level, field"] # [doc = " names, and optional source code location."] pub const fn new (name : & 'static str , target : & 'a str , level : Level , file : Option < & 'a str > , line : Option < u32 > , module_path : Option < & 'a str > , fields : field :: FieldSet , kind : Kind ,) -> Self { Metadata { name , target , level , module_path , file , line , fields , kind , } } # [doc = " Returns the names of the fields on the described span or event."] # [inline] pub fn fields (& self) -> & field :: FieldSet { & self . fields } # [doc = " Returns the level of verbosity of the described span or event."] pub fn level (& self) -> & Level { & self . level } # [doc = " Returns the name of the span."] pub fn name (& self) -> & 'static str { self . name } # [doc = " Returns a string describing the part of the system where the span or"] # [doc = " event that this metadata describes occurred."] # [doc = ""] # [doc = " Typically, this is the module path, but alternate targets may be set"] # [doc = " when spans or events are constructed."] pub fn target (& self) -> & 'a str { self . target } # [doc = " Returns the path to the Rust module where the span occurred, or"] # [doc = " `None` if the module path is unknown."] pub fn module_path (& self) -> Option < & 'a str > { self . module_path } # [doc = " Returns the name of the source code file where the span"] # [doc = " occurred, or `None` if the file is unknown"] pub fn file (& self) -> Option < & 'a str > { self . file } # [doc = " Returns the line number in the source code file where the span"] # [doc = " occurred, or `None` if the line number is unknown."] pub fn line (& self) -> Option < u32 > { self . line } # [doc = " Returns an opaque `Identifier` that uniquely identifies the callsite"] # [doc = " this `Metadata` originated from."] # [inline] pub fn callsite (& self) -> callsite :: Identifier { self . fields . callsite () } # [doc = " Returns true if the callsite kind is `Event`."] pub fn is_event (& self) -> bool { self . kind . is_event () } # [doc = " Return true if the callsite kind is `Span`."] pub fn is_span (& self) -> bool { self . kind . is_span () } }
    };
}

impl_193!();