macro_rules! EventKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] pub enum EventKind { StdTableOpen , StdTableClose , ArrayTableOpen , ArrayTableClose , InlineTableOpen , InlineTableClose , ArrayOpen , ArrayClose , SimpleKey , KeySep , KeyValSep , Scalar , ValueSep , Whitespace , Comment , Newline , Error , }
    };
}

EventKind!();