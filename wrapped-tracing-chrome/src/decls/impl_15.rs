macro_rules! deps {
    () => {
        JsonVisitor!();
        ChromeLayer!();
        TraceStyle!();
        ArgsWrapper!();
        EventOrSpan!();
        Message!();
        Object!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < S > Layer < S > for ChromeLayer < S > where S : Subscriber + for < 'span > LookupSpan < 'span > + Send + Sync , { fn on_enter (& self , id : & span :: Id , ctx : Context < '_ , S >) { if let TraceStyle :: Async = self . trace_style { return ; } let ts = self . get_ts () ; self . enter_span (ctx . span (id) . expect ("Span not found.") , ts) ; } fn on_record (& self , id : & span :: Id , values : & span :: Record < '_ > , ctx : Context < '_ , S >) { if self . include_args { let span = ctx . span (id) . unwrap () ; let mut exts = span . extensions_mut () ; let args = exts . get_mut :: < ArgsWrapper > () ; if let Some (args) = args { let args = Arc :: make_mut (& mut args . args) ; values . record (& mut JsonVisitor { object : args }) ; } } } fn on_event (& self , event : & Event < '_ > , _ctx : Context < '_ , S >) { let ts = self . get_ts () ; let callsite = self . get_callsite (EventOrSpan :: Event (event)) ; self . send_message (Message :: Event (ts , callsite)) ; } fn on_exit (& self , id : & span :: Id , ctx : Context < '_ , S >) { if let TraceStyle :: Async = self . trace_style { return ; } let ts = self . get_ts () ; self . exit_span (ctx . span (id) . expect ("Span not found.") , ts) ; } fn on_new_span (& self , attrs : & span :: Attributes < '_ > , id : & span :: Id , ctx : Context < '_ , S >) { if self . include_args { let mut args = Object :: new () ; attrs . record (& mut JsonVisitor { object : & mut args }) ; ctx . span (id) . unwrap () . extensions_mut () . insert (ArgsWrapper { args : Arc :: new (args) , }) ; } if let TraceStyle :: Threaded = self . trace_style { return ; } let ts = self . get_ts () ; self . enter_span (ctx . span (id) . expect ("Span not found.") , ts) ; } fn on_close (& self , id : span :: Id , ctx : Context < '_ , S >) { if let TraceStyle :: Threaded = self . trace_style { return ; } let ts = self . get_ts () ; self . exit_span (ctx . span (& id) . expect ("Span not found.") , ts) ; } }
    };
}

impl_15!();