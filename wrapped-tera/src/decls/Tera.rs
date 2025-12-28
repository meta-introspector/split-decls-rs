macro_rules! deps {
    () => {
        Function!();
        Template!();
        Test!();
        EscapeFn!();
        Filter!();
    };
}

macro_rules! Tera {
    () => {
        deps!();
        # [doc = " Main point of interaction in this library."] # [doc = ""] # [doc = " The [`Tera`] struct is the primary interface for working with the Tera template engine. It contains parsed templates, registered filters (which can filter"] # [doc = " data), functions, and testers. It also contains some configuration options, such as a list of"] # [doc = " suffixes for files that have autoescaping turned on."] # [doc = ""] # [doc = " It is responsible for:"] # [doc = ""] # [doc = " - Loading and managing templates from files or strings"] # [doc = " - Parsing templates and checking for syntax errors"] # [doc = " - Maintaining a cache of compiled templates for efficient rendering"] # [doc = " - Providing an interface for rendering templates with given contexts"] # [doc = " - Managing template inheritance and includes"] # [doc = " - Handling custom filters and functions"] # [doc = " - Overriding settings, such as autoescape rules"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use tera::Tera;"] # [doc = ""] # [doc = " // Create a new Tera instance and add a template from a string"] # [doc = " let mut tera = Tera::new(\"templates/**/*\").unwrap();"] # [doc = " tera.add_raw_template(\"hello\", \"Hello, {{ name }}!\").unwrap();"] # [doc = ""] # [doc = " // Prepare the context with some data"] # [doc = " let mut context = tera::Context::new();"] # [doc = " context.insert(\"name\", \"World\");"] # [doc = ""] # [doc = " // Render the template with the given context"] # [doc = " let rendered = tera.render(\"hello\", &context).unwrap();"] # [doc = " assert_eq!(rendered, \"Hello, World!\");"] # [doc = " ```"] # [derive (Clone)] pub struct Tera { # [doc (hidden)] glob : Option < String > , # [doc (hidden)] pub templates : HashMap < String , Template > , # [doc (hidden)] pub filters : HashMap < String , Arc < dyn Filter > > , # [doc (hidden)] pub testers : HashMap < String , Arc < dyn Test > > , # [doc (hidden)] pub functions : HashMap < String , Arc < dyn Function > > , # [doc (hidden)] pub autoescape_suffixes : Vec < & 'static str > , # [doc (hidden)] escape_fn : EscapeFn , }
    };
}

Tera!()