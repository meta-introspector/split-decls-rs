// Generated macro for State (enum)
macro_rules! Depcrate_parserState {
() => {
// Module: crate::parser
// Provides: {"State"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq , Debug , Eq)] enum State { # [doc = " We await the start of the stream."] StreamStart , ImplicitDocumentStart , DocumentStart , DocumentContent , DocumentEnd , BlockNode , BlockSequenceFirstEntry , BlockSequenceEntry , IndentlessSequenceEntry , BlockMappingFirstKey , BlockMappingKey , BlockMappingValue , FlowSequenceFirstEntry , FlowSequenceEntry , FlowSequenceEntryMappingKey , FlowSequenceEntryMappingValue , FlowSequenceEntryMappingEnd , FlowMappingFirstKey , FlowMappingKey , FlowMappingValue , FlowMappingEmptyValue , End , }
};
}
