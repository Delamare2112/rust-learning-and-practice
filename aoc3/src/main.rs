use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Default,Copy,Clone)]
struct Triangle {
    a: usize,
    b: usize,
    c: usize
}
impl Triangle {
    fn is_valid(&self) -> bool {
        let mut max;
        let mut len_other;
        if self.a > self.b {
            len_other=self.b;
            max = self.a;
        } else {
            len_other=self.a;
            max = self.b;
        }
        if self.c > max {
            max=self.c;
            len_other = self.a + self.b;
        } else {
            len_other += self.c;
        }
        len_other > max
    }
}

fn part1(file: &File) -> usize {
    let mut result = 0usize;
    let file = BufReader::new(file);
    for line in file.lines().filter_map(|result| result.ok()) {
        let mut words = line.split(" ").map(|x| usize::from_str_radix(x, 10).unwrap());
        let t = Triangle{a: words.next().unwrap(), b: words.next().unwrap(), c: words.next().unwrap()};
        if t.is_valid() {
            result += 1;
        }
    }
    result
}

fn part2(mut file: &File) -> usize {
    use std::io::Read;
    let mut result = 0usize;
    let mut str_data = String::new();
    file.read_to_string(&mut str_data).expect("shit");
    let mut lines = str_data.split("\n").peekable();
    while lines.peek() != None {
        let mut triangles = [Triangle::default(); 3];

        let mut words = lines.next().unwrap().split(" ").map(|x| usize::from_str_radix(x, 10).unwrap());
        for t in triangles.iter_mut() {
            t.a = words.next().unwrap();
        }
        let mut words = lines.next().unwrap().split(" ").map(|x| usize::from_str_radix(x, 10).unwrap());
        for t in triangles.iter_mut() {
            t.b = words.next().unwrap();
        }
        let mut words = lines.next().unwrap().split(" ").map(|x| usize::from_str_radix(x, 10).unwrap());
        for t in triangles.iter_mut() {
            t.c = words.next().unwrap();
        }

        for t in triangles.iter() {
            if t.is_valid() {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    use std::io::Seek;
    let mut file = File::open("/home/delamare/rust/aoc3/src/input.txt").expect("cannot open input file");
    println!("{}", part1(&file).to_string());
    file.seek(std::io::SeekFrom::Start(0));
    println!("{}", part2(&file).to_string());
}
