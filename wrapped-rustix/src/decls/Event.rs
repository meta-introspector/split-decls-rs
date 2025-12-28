macro_rules! Event {
    () => {
        # [doc = " An inotify event."] # [doc (alias = "inotify_event")] # [derive (Debug)] pub struct Event < 'a > { wd : i32 , events : ReadFlags , cookie : u32 , file_name : Option < & 'a CStr > , }
    };
}

Event!();