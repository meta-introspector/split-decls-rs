// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_11",
decl_type: "function",
source_file: "./src/line_counter.rs",
source_crate: ".",
deps: ["LineCountReport", "ErrorItem", "Output", "Input", "SkippedItem"],
uses: ["LineCountReport", "ErrorItem", "SKIPPED", "ITEMS", "ERROR", "Skipped", "Output", "Input", "Vec", "REPORT", "Error", "Coverage", "LINE", "Processed", "COUNT", "SkippedItem", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        LineCountReport!();
        ErrorItem!();
        Output!();
        Input!();
        SkippedItem!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl LineCountReport { pub fn new () -> Self { Self :: default () } pub fn add_input_lines (& mut self , lines : usize) { self . input_lines += lines ; } pub fn add_output_lines (& mut self , lines : usize) { self . output_lines += lines ; } pub fn add_skipped_item (& mut self , item_type : String , reason : String , content : & str) { let preview = content . lines () . take (3) . collect :: < Vec < _ > > () . join (" ") ; self . skipped_items . push (SkippedItem { item_type , reason , content_preview : preview . chars () . take (100) . collect () , }) ; } pub fn add_error_item (& mut self , item_type : String , error : String , content : & str) { let preview = content . lines () . take (3) . collect :: < Vec < _ > > () . join (" ") ; self . error_items . push (ErrorItem { item_type , error_message : error , content_preview : preview . chars () . take (100) . collect () , }) ; } pub fn add_processed_item (& mut self) { self . processed_items += 1 ; } pub fn print_summary (& self) { println ! ("\n📊 LINE COUNT REPORT") ; println ! ("==================") ; println ! ("📥 Input lines:     {}" , self . input_lines) ; println ! ("📤 Output lines:    {}" , self . output_lines) ; println ! ("✅ Processed items: {}" , self . processed_items) ; println ! ("⚠️  Skipped items:   {}" , self . skipped_items . len ()) ; println ! ("❌ Error items:     {}" , self . error_items . len ()) ; let coverage = if self . input_lines > 0 { (self . output_lines as f64 / self . input_lines as f64) * 100.0 } else { 0.0 } ; println ! ("📈 Coverage:        {:.1}%" , coverage) ; if ! self . skipped_items . is_empty () { println ! ("\n⚠️  SKIPPED ITEMS:") ; for (i , item) in self . skipped_items . iter () . enumerate () { println ! ("  {}. {} - {} | {}" , i + 1 , item . item_type , item . reason , item . content_preview) ; } } if ! self . error_items . is_empty () { println ! ("\n❌ ERROR ITEMS:") ; for (i , item) in self . error_items . iter () . enumerate () { println ! ("  {}. {} - {} | {}" , i + 1 , item . item_type , item . error_message , item . content_preview) ; } } } }
    };
}

impl_11!();