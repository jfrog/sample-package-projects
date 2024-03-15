use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

/// Static array of Yogi Berra quotes.
static YOGI_QUOTES: &[&str] = &[
    "It ain't over till it's over.",
    "When you come to a fork in the road, take it.",
    "You can observe a lot by just watching.",
    "No one goes there nowadays, it’s too crowded.",
    "Baseball is 90% mental and the other half is physical.",
    "Always go to other people’s funerals, otherwise they won’t go to yours.",
    "The future ain’t what it used to be.",
];

/// Returns a random Yogi Berra quote.
pub fn get_random_quote() -> &'static str {
    let mut rng = thread_rng();
    YOGI_QUOTES.choose(&mut rng).unwrap_or(&"No quote found.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_quote() {
        let quote = get_random_quote();
        assert!(YOGI_QUOTES.contains(&quote));
    }
}