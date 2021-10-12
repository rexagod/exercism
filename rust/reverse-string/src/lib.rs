use unicode_segmentation::UnicodeSegmentation;
// had to do this to get it work (some issue with cargo cli not using git)
// export CARGO_NET_GIT_FETCH_WITH_CLI=true

pub fn reverse(input: &str) -> String {
    let mut output: String = "".to_owned();
    for elem in input.graphemes(true) {
        output = format!("{}{}", elem, output)
    }
    return output
}
