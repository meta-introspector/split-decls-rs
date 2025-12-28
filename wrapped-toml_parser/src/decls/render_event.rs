macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! render_event {
    () => {
        deps!();
        fn render_event (span : impl Into < Option < Span > > , text : & str , style : anstyle :: Style) { # ! [allow (unexpected_cfgs)] let span = span . into () ; let depth = DEBUG_DEPTH . depth () . min (20) ; anstream :: eprintln ! ("{:depth$}{style}{text}: {span:?}{style:#}" , "") ; }
    };
}

render_event!();