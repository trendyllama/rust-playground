

pub fn playing_w_vectors() {


    let v = vec![1, 2, 3, 4, 5];


    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);
    v2.push(5);

    println!("v2: {:?}", v2);
    println!("v: {:?}", v);

    assert_eq!(v, v2);

}

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