macro_rules! deps {
    () => {
        Context!();
        Tera!();
        Template!();
        Result!();
        Processor!();
        Renderer!();
    };
}

macro_rules! impl_533 {
    () => {
        deps!();
        impl < 'a > Renderer < 'a > { # [doc = " Create a new `Renderer`"] # [inline] pub fn new (template : & 'a Template , tera : & 'a Tera , context : & 'a Context) -> Renderer < 'a > { let should_escape = tera . autoescape_suffixes . iter () . any (| ext | { if let Some (ref p) = template . path { return p . ends_with (ext) ; } template . name . ends_with (ext) }) ; Renderer { template , tera , context , should_escape } } # [doc = " Combines the context with the Template to generate the end result"] pub fn render (& self) -> Result < String > { let mut output = Vec :: with_capacity (2000) ; self . render_to (& mut output) ? ; buffer_to_string (| | "converting rendered buffer to string" . to_string () , output) } # [doc = " Combines the context with the Template to write the end result to output"] pub fn render_to (& self , mut output : impl Write) -> Result < () > { let mut processor = Processor :: new (self . template , self . tera , self . context , self . should_escape) ; processor . render (& mut output) } }
    };
}

impl_533!()