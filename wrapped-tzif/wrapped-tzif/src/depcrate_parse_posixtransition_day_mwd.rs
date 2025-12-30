// Generated macro for transition_day_mwd (function)
macro_rules! Depcrate_parse_posixtransition_day_mwd {
() => {
// Module: crate::parse::posix
// Provides: {"transition_day_mwd"}
// Dependencies: {}
# [doc = " Parses a transition date specified by a leading `M`, e.g. `Mm.w.d`"] # [doc = " This specifies day `d` of week `w` of month `m`. The day `d` must be between"] # [doc = " 0 (Sunday) and 6 (Saturday). The week `w` must be in range `[1, 5]`;"] # [doc = " The week that corresponds to `1` is the first week in which day `d` occurs,"] # [doc = " and week 5 specifies the last `d` day in the month. The month `m` should"] # [doc = " be between 1 and 12."] fn transition_day_mwd < Input > () -> impl Parser < Input , Output = TransitionDay > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { byte (b'M') . with ((bounded_natural (1 , 12) , byte (b'.') . with (bounded_natural (1 , 5)) , byte (b'.') . with (bounded_natural (0 , 6)) ,)) . map (| (m , w , d) | TransitionDay :: Mwd (m as u16 , w as u16 , d as u16)) }
};
}
