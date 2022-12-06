use std::fs;
use std::collections::BTreeSet;

const ERROR_MESSAGE: &str = "Some weird error happened. Help!";

fn main() {
    let file = fs::read_to_string("input").expect(ERROR_MESSAGE);
    let answer = file
      .lines()
      .fold(0, |acc, line| {
        let length = line.chars().count();
        let mut c = 0;
        let (left, right): (Vec<u8>, Vec<u8>) = line
          .chars()
          .map(|c| match c {
            'a' ..='z' => (c as u8) + 1 - 'a' as u8,
            'A' ..='Z' => (c as u8) + 27 - 'A' as u8,
            _ => unreachable!()
          })
          .partition(|_| {
            let result = c < length.saturating_div(2);
            c += 1;
            result
          });
        let left_set = BTreeSet::from_iter(left);
        let right_set = BTreeSet::from_iter(right);
        let count: u32 = right_set.into_iter().fold(0, |acc, item| {
          acc + if left_set.contains(&item) {
            item as u32
          } else {
            0
          }
        });
        acc + count
      });
    println!("Answer: {:?}", answer);
  }
