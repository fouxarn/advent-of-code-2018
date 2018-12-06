use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn execute() {
  let mut f = File::open("src/day01.input").expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");;
  let input = contents.trim();

  println!("Day1_part1: {}", solve1(input));
  println!("Day1_part2: {}", solve2(input));
}

fn solve1(input: &str) -> i32 {
  // Split the input lines, and filter empty lines
  let mut rows = input
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty());

  let mut result = 0;

  loop {
    match rows.next() {
      Some(x) => {
        result += x.parse::<i32>().unwrap();
      }
      None => break,
    }
  }

  return result;
}

fn solve2(input: &str) -> i32 {
  // Split the input lines, and filter empty lines
  let mut rows = input
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .cycle();

  let mut frequencies = HashMap::new();
  let mut result = 0;
  frequencies.insert(0, true);

  loop {
    match rows.next() {
      Some(x) => {
        result += x.parse::<i32>().unwrap();
        if frequencies.insert(result, true) != None {
          return result;
        };
      }
      None => break,
    }
  }

  return 0;
}

#[test]
fn day01() {
  assert_eq!(
    solve1(
      "
    +1
    +1
    +1
    "
    ),
    3
  );

  assert_eq!(
    solve1(
      "
    +1
    +1
    -2
    "
    ),
    0
  );

  assert_eq!(
    solve1(
      "
    -1
    -2
    -3
    "
    ),
    -6
  );

  assert_eq!(
    solve2(
      "
    +1
    -1
    "
    ),
    0
  );

  assert_eq!(
    solve2(
      "
    +3
    +3
    +4
    -2
    -4
    "
    ),
    10
  );

  assert_eq!(
    solve2(
      "
    -6
    +3
    +8
    +5
    -6
    "
    ),
    5
  );

  assert_eq!(
    solve2(
      "
    +7
    +7
    -2
    -7
    -4
    "
    ),
    14
  );
}
