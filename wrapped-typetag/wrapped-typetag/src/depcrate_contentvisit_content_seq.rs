// Generated macro for visit_content_seq (function)
macro_rules! Depcrate_contentvisit_content_seq {
() => {
// Module: crate::content
// Provides: {"visit_content_seq"}
// Dependencies: {}
fn visit_content_seq < 'de , V , E > (content : Vec < Content < 'de > > , visitor : V) -> Result < V :: Value , E > where V : Visitor < 'de > , E : de :: Error , { let seq = content . into_iter () . map (ContentDeserializer :: new) ; let mut seq_visitor = value :: SeqDeserializer :: new (seq) ; let value = visitor . visit_seq (& mut seq_visitor) ? ; seq_visitor . end () ? ; Ok (value) }
};
}
