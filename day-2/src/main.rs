use std::fs;
use itertools::Itertools;

const ERROR_MESSAGE: &str = "Some weird error happened. Help!";

fn main() {
    let file = fs::read_to_string("input").expect("ok");
    let answer = file
      .lines()
      .fold(0, |acc, line| {
        let i = line
          .split(' ')
          .map(|v| v.chars().nth(0).expect(ERROR_MESSAGE) as i8);
        if let Some((opponent, me)) = i.collect_tuple() {
          let opponent_choice = opponent - 'A' as i8;
          let my_choice = me - 'X' as i8;
          let mut score: u32 = match my_choice.saturating_sub(opponent_choice).wrapping_rem_euclid(3) {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => unreachable!()
          };
          score += my_choice as u32 + 1;
          acc + score
        } else {
          unreachable!()
        }
      });
    println!("Answer: {:?}", answer);
  }
