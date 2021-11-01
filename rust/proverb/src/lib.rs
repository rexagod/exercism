pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb: String = "".to_owned();
    for i in 1..=list.len() {
        if i == list.len() {
            proverb = format!("{}And all for the want of a {}.", proverb, list[0])
        } else {
            proverb = format!("{}For want of a {} the {} was lost.\n", proverb, list[i-1], list[i])
        }
    }
    proverb.to_owned()
}
