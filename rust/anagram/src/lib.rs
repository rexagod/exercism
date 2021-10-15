use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut ans: HashSet<&'a str> = HashSet::new();
    let mut word_vec: Vec<String> = word.split("")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.to_lowercase()))
        .collect::<Vec<String>>();
    let word_vec_unsorted: Vec<String> = word_vec.to_owned();
    word_vec.sort_unstable();
    print!("Word {}, {:?}\n", word, word_vec);

    for &candidate in possible_anagrams {
        let mut candidate_vec: Vec<String> = candidate.trim().split("")
            .filter(|x| !x.is_empty())
            .map(|x| String::from(x.to_lowercase()))
            .collect::<Vec<String>>();
        print!("Candidates {}, {:?}\n", candidate, candidate_vec);
        if !word_vec_unsorted.eq(&candidate_vec) {
            candidate_vec.sort_unstable();
            if word_vec.eq(&candidate_vec) && (*word) != (*candidate) {
                ans.insert(candidate);
            }
        }
    }
    return ans;
}
