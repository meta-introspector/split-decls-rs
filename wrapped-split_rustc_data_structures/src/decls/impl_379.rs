macro_rules! deps {
    () => {
        JsonTimePassesEntry!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl Display for JsonTimePassesEntry < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let Self { pass : what , time , start_rss , end_rss } = self ; write ! (f , r#"{{"pass":"{what}","time":{time},"rss_start":"#) . unwrap () ; match start_rss { Some (rss) => write ! (f , "{rss}") ? , None => write ! (f , "null") ? , } write ! (f , r#","rss_end":"#) ? ; match end_rss { Some (rss) => write ! (f , "{rss}") ? , None => write ! (f , "null") ? , } write ! (f , "}}") ? ; Ok (()) } }
    };
}

impl_379!()