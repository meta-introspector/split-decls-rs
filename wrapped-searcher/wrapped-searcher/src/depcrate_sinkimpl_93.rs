// Generated macro for impl_93 (impl)
macro_rules! Depcrate_sinkimpl_93 {
() => {
// Module: crate::sink
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'a , S : Sink > Sink for & 'a mut S { type Error = S :: Error ; # [inline] fn matched (& mut self , searcher : & Searcher , mat : & SinkMatch < '_ > ,) -> Result < bool , S :: Error > { (* * self) . matched (searcher , mat) } # [inline] fn context (& mut self , searcher : & Searcher , context : & SinkContext < '_ > ,) -> Result < bool , S :: Error > { (* * self) . context (searcher , context) } # [inline] fn context_break (& mut self , searcher : & Searcher ,) -> Result < bool , S :: Error > { (* * self) . context_break (searcher) } # [inline] fn binary_data (& mut self , searcher : & Searcher , binary_byte_offset : u64 ,) -> Result < bool , S :: Error > { (* * self) . binary_data (searcher , binary_byte_offset) } # [inline] fn begin (& mut self , searcher : & Searcher) -> Result < bool , S :: Error > { (* * self) . begin (searcher) } # [inline] fn finish (& mut self , searcher : & Searcher , sink_finish : & SinkFinish ,) -> Result < () , S :: Error > { (* * self) . finish (searcher , sink_finish) } }
};
}
