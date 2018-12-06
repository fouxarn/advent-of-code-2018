use std::fs::File;
use std::io::Read;

pub fn execute() {
  let mut f = File::open("src/day03.input").expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");;
  let input = contents.trim();

  println!("Day3_part1: {}", solve1(input));
  println!("Day3_part2: {}", solve2(input));
}

fn solve1(input: &str) -> i32 {
  // Split the input lines, and filter empty lines
  let rows = input
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty());

  let mut fabric = vec![vec![0; 2000]; 2000];

  let mut result = 0;

  for row in rows {
    let yo: Vec<&str> = row.split(" ").collect();

    let pos: Vec<u16> = yo[2]
      .split(|x| x == ',' || x == ':')
      .map(|x| x.parse::<u16>())
      .filter_map(|x| x.ok())
      .collect();
    let size: Vec<u16> = yo[3]
      .split("x")
      .map(|x| x.parse::<u16>())
      .filter_map(|x| x.ok())
      .collect();

    for x in 0..size[0] {
      for y in 0..size[1] {
        fabric[(x + pos[0]) as usize][(y + pos[1]) as usize] += 1;

        if fabric[(x + pos[0]) as usize][(y + pos[1]) as usize] == 2 {
          result += 1;
        }
      }
    }
  }

  result
}

fn solve2(input: &str) -> &str {
  // Split the input lines, and filter empty lines
  let rows = input
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();

  let mut fabric = vec![vec![0; 2000]; 2000];

  for row in &rows {
    let yo: Vec<&str> = row.split(" ").collect();

    let pos: Vec<u16> = yo[2]
      .split(|x| x == ',' || x == ':')
      .map(|x| x.parse::<u16>())
      .filter_map(|x| x.ok())
      .collect();
    let size: Vec<u16> = yo[3]
      .split("x")
      .map(|x| x.parse::<u16>())
      .filter_map(|x| x.ok())
      .collect();

    for x in 0..size[0] {
      for y in 0..size[1] {
        fabric[(x + pos[0]) as usize][(y + pos[1]) as usize] += 1;
      }
    }
  }

  'outer: for row in &rows {
    let yo: Vec<&str> = row.split(" ").collect();

    let temp: Vec<&str> = yo[0].split('#').collect();
    let id = temp[1];

    let pos: Vec<usize> = yo[2]
      .split(|x| x == ',' || x == ':')
      .map(|x| x.parse::<usize>())
      .filter_map(|x| x.ok())
      .collect();
    let size: Vec<usize> = yo[3]
      .split("x")
      .map(|x| x.parse::<usize>())
      .filter_map(|x| x.ok())
      .collect();

    let mut found = false;

    for x in 0..size[0] {
      for y in 0..size[1] {
        if fabric[(x + pos[0]) as usize][(y + pos[1]) as usize] == 1 {
          found = true;
        } else {
          continue 'outer;
        }
      }
    }

    if found == true {
      return id;
    }
  }

  "0"
}

#[test]
fn test() {
  assert_eq!(
    solve1(
      "
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
    ),
    4
  );

  assert_eq!(
    solve2(
      "
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
    ),
    "3"
  );
}
