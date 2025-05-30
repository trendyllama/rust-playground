pub fn playing_w_vectors() {
    let _v = [1, 2, 3, 4, 5];
}

#[allow(dead_code)]
pub fn playing_w_hashmaps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    println!("Scores: {:?}", scores);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playing_w_vectors() {
        playing_w_vectors();
    }

    #[test]
    fn test_playing_w_hashmaps() {
        playing_w_hashmaps();
    }
}
