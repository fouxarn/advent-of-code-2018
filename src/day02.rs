use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn execute() {
  let mut f = File::open("src/day02.input").expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");;
  let input = contents.trim();

  println!("Day2_part1: {}", solve1(input));
  println!("Day2_part2: {}", solve2(input));
}

fn solve1(input: &str) -> i32 {
  // Split the input lines, and filter empty lines
  let rows = input
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty());

  let mut twos = 0;
  let mut threes = 0;

  for row in rows {
    let b = calc_box(row);
    if b.twos {
      twos += 1;
    }
    if b.threes {
      threes += 1;
    }
  }

  twos * threes
}

fn solve2(input: &str) -> String {
  // Split the input lines, and filter empty lines
  let rows = input
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();

  //let otherRows: Vec<_> = rows.iter().cloned().collect();

  for row in &rows {
    for other_row in &rows {
      let (dist, change) = diff(row, other_row);
      if dist == 1 {
        return change;
      }
    }
  }

  "".to_string()
}

fn diff(s1: &str, s2: &str) -> (u32, String) {
  let mut distance = 0;
  let mut change = String::from("");

  if s1 != s2 && s1.len() == s2.len() {
    let mut other = s2.chars();

    for c in s1.chars() {
      if c != other.next().unwrap() {
        distance += 1;
      } else {
        change.push(c);
      }
    }
  }

  (distance, change)
}

#[derive(Debug)]
struct ChecksumBox {
  twos: bool,
  threes: bool,
}

impl PartialEq for ChecksumBox {
  fn eq(&self, other: &ChecksumBox) -> bool {
    self.twos == other.twos && self.threes == other.threes
  }
}

fn calc_box(cbox: &str) -> ChecksumBox {
  let mut frequency = HashMap::new();

  for c in cbox.chars() {
    *frequency.entry(c).or_insert(0) += 1;
  }

  let mut result = ChecksumBox {
    twos: false,
    threes: false,
  };
  for val in frequency.values() {
    match val {
      2 => result.twos = true,
      3 => result.threes = true,
      _ => (),
    }
  }

  return result;
}

#[test]
fn day02() {
  assert_eq!(
    calc_box("abcdef"),
    ChecksumBox {
      twos: false,
      threes: false,
    }
  );

  assert_eq!(
    calc_box("bababc"),
    ChecksumBox {
      twos: true,
      threes: true,
    }
  );

  assert_eq!(
    calc_box("abbcde"),
    ChecksumBox {
      twos: true,
      threes: false,
    }
  );

  assert_eq!(
    calc_box("abcccd"),
    ChecksumBox {
      twos: false,
      threes: true,
    }
  );

  assert_eq!(
    calc_box("aabcdd"),
    ChecksumBox {
      twos: true,
      threes: false,
    }
  );

  assert_eq!(
    calc_box("abcdee"),
    ChecksumBox {
      twos: true,
      threes: false,
    }
  );

  assert_eq!(
    calc_box("ababab"),
    ChecksumBox {
      twos: false,
      threes: true,
    }
  );
}
