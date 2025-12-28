macro_rules! deps {
    () => {
        EventKind!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl EventKind { pub const fn description (& self) -> & 'static str { match self { Self :: StdTableOpen => "std-table open" , Self :: StdTableClose => "std-table close" , Self :: ArrayTableOpen => "array-table open" , Self :: ArrayTableClose => "array-table close" , Self :: InlineTableOpen => "inline-table open" , Self :: InlineTableClose => "inline-table close" , Self :: ArrayOpen => "array open" , Self :: ArrayClose => "array close" , Self :: SimpleKey => "key" , Self :: KeySep => "key separator" , Self :: KeyValSep => "key-value separator" , Self :: Scalar => "value" , Self :: ValueSep => "value separator" , Self :: Whitespace => "whitespace" , Self :: Comment => "comment" , Self :: Newline => "newline" , Self :: Error => "error" , } } }
    };
}

impl_192!()