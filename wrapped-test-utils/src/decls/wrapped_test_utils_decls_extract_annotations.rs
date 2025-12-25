use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Extracts `//^^^ some text` annotations.
///
/// A run of `^^^` can be arbitrary long and points to the corresponding range
/// in the line above.
///
/// The `// ^file text` syntax can be used to attach `text` to the entirety of
/// the file.
///
/// Multiline string values are supported:
///
/// // ^^^ first line
/// //   | second line
///
/// Trailing whitespace is sometimes desired but usually stripped by the editor
/// if at the end of a line, or incorrectly sized if followed by another
/// annotation. In those cases the annotation can be explicitly ended with the
/// `$` character.
///
/// // ^^^ trailing-ws-wanted  $
///
/// Annotations point to the last line that actually was long enough for the
/// range, not counting annotations themselves. So overlapping annotations are
/// possible:
/// ```text
/// // stuff        other stuff
/// // ^^ 'st'
/// // ^^^^^ 'stuff'
/// //              ^^^^^^^^^^^ 'other stuff'
/// ```
pub fn extract_annotations(text: &str) -> Vec<(TextRange, String)> {
    let mut res = Vec::new();
    let mut line_start_map = BTreeMap::new();
    let mut line_start: TextSize = 0.into();
    let mut prev_line_annotations: Vec<(TextSize, usize)> = Vec::new();
    for line in text.split_inclusive('\n') {
        let mut this_line_annotations = Vec::new();
        let line_length = if let Some((prefix, suffix)) = line.split_once("//") {
            let ss_len = TextSize::of("//");
            let annotation_offset = TextSize::of(prefix) + ss_len;
            for annotation in extract_line_annotations(suffix.trim_end_matches('\n')) {
                match annotation {
                    LineAnnotation::Annotation { mut range, content, file } => {
                        range += annotation_offset;
                        this_line_annotations.push((range.end(), res.len()));
                        let range = if file {
                            TextRange::up_to(TextSize::of(text))
                        } else {
                            let line_start = line_start_map
                                .range(range.end()..)
                                .next()
                                .unwrap();
                            range + line_start.1
                        };
                        res.push((range, content));
                    }
                    LineAnnotation::Continuation { mut offset, content } => {
                        offset += annotation_offset;
                        let &(_, idx) = prev_line_annotations
                            .iter()
                            .find(|&&(off, _idx)| off == offset)
                            .unwrap();
                        res[idx].1.push('\n');
                        res[idx].1.push_str(&content);
                        res[idx].1.push('\n');
                    }
                }
            }
            annotation_offset
        } else {
            TextSize::of(line)
        };
        line_start_map = line_start_map.split_off(&line_length);
        line_start_map.insert(line_length, line_start);
        line_start += TextSize::of(line);
        prev_line_annotations = this_line_annotations;
    }
    res
}
