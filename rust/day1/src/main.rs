use std::fs;

#[derive(Debug)]
struct TrieNode {
    next: std::collections::HashMap<char, TrieNode>,
    val: Option<u32>,
}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            next: std::collections::HashMap::new(),
            val: None,
        }
    }

    // A convenience method for constructing a trie with the digits from one through nine included
    fn with_digits() -> TrieNode {
        let mut trie = TrieNode::new();
        trie.insert("one", 1);
        trie.insert("1", 1);
        trie.insert("two", 2);
        trie.insert("2", 2);
        trie.insert("three", 3);
        trie.insert("3", 3);
        trie.insert("four", 4);
        trie.insert("4", 4);
        trie.insert("five", 5);
        trie.insert("5", 5);
        trie.insert("six", 6);
        trie.insert("6", 6);
        trie.insert("seven", 7);
        trie.insert("7", 7);
        trie.insert("eight", 8);
        trie.insert("8", 8);
        trie.insert("nine", 9);
        trie.insert("9", 9);

        trie
    }

    fn insert(&mut self, key: &str, val: u32) {
        match key.chars().next() {
            // There is more to insert, take the first character as the key and insert the rest of
            // the string recursively as new trie nodes
            Some(ch) => {
                match self.next.get_mut(&ch) {
                    // If we have this key already, we pass the rest of the string to the
                    // corresponding TrieNode
                    Some(node) => node.insert(&key[1..], val),
                    None => {
                        // Otherwise, we create a new TrieNode and insert it as a branch of our own
                        // Node
                        let mut new_node = TrieNode::new();
                        new_node.insert(&key[1..], val);
                        self.next.insert(ch, new_node);
                    }
                }
            }
            // We reached the end of the key string and now we can insert our value
            None => self.val = Some(val),
        }
    }

    // Return a digit if the the next characters spell the name of a digit, otherwise None. Also
    // returns the number of characters read
    fn get_digit(&self, chars: &mut std::str::Chars, read_count: u32) -> (Option<u32>, u32) {
        self.val.map_or(
            match chars.next() {
                Some(ch) => {
                    //println!("checking digit on {ch}");

                    self.next
                        .get(&ch)
                        .map_or_else(|| (None, 0), |child| child.get_digit(chars, read_count + 1))
                }
                None => (None, 0),
            },
            |val| (Some(val), read_count),
        )
    }
}

fn get_until_digit(trie: &TrieNode, chars: &mut std::str::Chars) -> Option<u32> {
    loop {
        let (digit, read_count) = trie.get_digit(&mut chars.clone(), 0);
        match digit {
            Some(digit) => {
                // advance position past what we just read
                for _ in 0..read_count {
                    chars.next();
                }
                break Some(digit);
            }
            _ => {
                // Advance by one character and if we're at the end of the string return
                if chars.next().is_none() {
                    break None;
                }
            }
        }
    }
}

fn calibrate(trie: &TrieNode, line: &str) -> u32 {
    let mut chars = line.chars();
    let first = get_until_digit(trie, &mut chars).unwrap_or(0);
    //println!("calibrate(): got first digit {first}");
    let mut last = first;
    while let Some(digit) = get_until_digit(trie, &mut chars) {
        //println!("calibrate(): got some digit {digit}");
        last = digit;
    }

    first * 10 + last
}

fn main() {
    let inpfile = "./input.txt";

    let file_contents = fs::read_to_string(inpfile).expect("Failed to read input file!");

    let trie = TrieNode::with_digits();

    let mut sum = 0;
    let mut line_no = 0;
    for line in file_contents.lines() {
        let total = calibrate(&trie, line);
        sum += total;
        println!("checking line {line_no}: {line} total={total} sum={sum}");
        line_no += 1;
    }

    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate() {
        let trie = TrieNode::with_digits();
        assert_eq!(calibrate(&trie, "1abc2"), 12);
        assert_eq!(calibrate(&trie, "pqr3stu8vwx"), 38);
        assert_eq!(calibrate(&trie, "a1b2c3d4e5f"), 15);
        assert_eq!(calibrate(&trie, "treb7uchet"), 77);
        assert_eq!(calibrate(&trie, "treb7uchet"), 77);
        assert_eq!(calibrate(&trie, "two1nine"), 29);
        assert_eq!(calibrate(&trie, "eightwothree"), 83);
        assert_eq!(calibrate(&trie, "abcone2threexyz"), 13);
        assert_eq!(calibrate(&trie, "xtwone3four"), 24);
        assert_eq!(calibrate(&trie, "zoneight234"), 14);
        assert_eq!(calibrate(&trie, "7pqrstsixteen"), 76);
        assert_eq!(calibrate(&trie, "4nineeightseven2"), 42);
        assert_eq!(calibrate(&trie, "53sdthreeninexrfone"), 51);
        assert_eq!(calibrate(&trie, "threseven9"), 79);
        assert_eq!(calibrate(&trie, "2hreseven98"), 28);
        assert_eq!(calibrate(&trie, "thresevennin"), 77);
        assert_eq!(calibrate(&trie, "hwqesaasd"), 0);
        assert_eq!(calibrate(&trie, "fjdsgcsqppzdthreefour3one3lvmpm"), 33);
    }

    #[test]
    fn test_trie() {
        let mut trie = TrieNode::new();
        assert_eq!(trie.get_digit(&mut "".chars(), 0), (None, 0));

        trie.insert("seven", 7);
        trie.insert("nine", 9);

        assert_eq!(trie.get_digit(&mut "seven".chars(), 0), (Some(7), 5));

        trie.insert("7", 7);
        assert_eq!(trie.get_digit(&mut "7".chars(), 0), (Some(7), 1));
    }
}
