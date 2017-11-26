use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    sector_id: usize,
    checksum: String
}
impl Room {
    fn is_real(&self) -> bool {
        // count
        let mut occurrences: HashMap<char, usize> = HashMap::new();
        for c in self.encrypted_name.chars().filter(|x| *x != '-') {
            *occurrences.entry(c).or_insert(0) += 1;
        }
        // sort
        let mut occurrences: Vec<(&char, &usize)> = occurrences.iter().collect();
        occurrences.sort_by(|a,b| {
            let mut x = b.1.cmp(a.1);
            if x == std::cmp::Ordering::Equal {
                x = a.0.cmp(b.0);
            }
            x
        });
        // validate
        let mut valid = true;
        let mut checksum_chars = self.checksum.chars().peekable();
        for (count, c) in occurrences.iter().enumerate() {
            if checksum_chars.peek() == None {
                break;
            }
            if *c.0 != checksum_chars.next().unwrap() {
                valid = false;
                break;
            }
        }
        valid
    }

    fn decrypt(&self) -> String {
        let mut result = String::new();
        for c in self.encrypted_name.chars() {
            if c == '-' {
                result.push(' ');
                continue;
            }
            let more_chars = (self.sector_id % 26) as u8;
            let nc = c as u8 + more_chars;
            if nc > 'z' as u8 {
                result.push(('a' as u8 + (nc as u8 - 'z' as u8) - 1) as char);
            }
            else {
                result.push(nc as char);
            }
        }
        result
    }
}

fn main() {
    let file = File::open("/home/delamare/rust/aoc4/src/input.txt").expect("cannot open input file");
    let file = BufReader::new(file);
    let mut sum = 0usize;
    for line in file.lines().filter_map(|x| x.ok()) {
        let last_dash = line.rfind("-").unwrap();
        let first_bracket = line.find("[").unwrap();
        let room = Room {
            encrypted_name: line[0..last_dash].to_string(),
            checksum: String::from(line[first_bracket+1..first_bracket+6].as_ref()),
            sector_id: line[last_dash+1..first_bracket].parse().unwrap()
        };
        if room.is_real() {
            sum += room.sector_id;
            if room.decrypt() == "northpole object storage" {
                println!("North Pole = {}", room.sector_id);
            }
        }
    }
    println!("{}", sum);
}
