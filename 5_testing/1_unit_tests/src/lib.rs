use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut result = "".to_string();
    let length = input.graphemes(true).count();

    for i in 0..length {
        let position = length - i;
        result = result + input.grapheme_indices(true)
                .nth(position)
                .expect("contains character at index").1;
    }

    result
}
