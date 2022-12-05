use std::{fs, ops::AddAssign};

const ERROR_MESSAGE: &str = "Some weird error happened. Help!";

fn main() {
    let file = fs::read_to_string("input").expect("ok");
    let initial_state: Vec<u32> = vec![0];
    let answer = file
      .lines()
      .fold(initial_state, |mut acc, line| {
        match line.parse::<u32>() {
          Ok(calories) => {
            acc.last_mut().expect(ERROR_MESSAGE).add_assign(calories);
            acc
          },
          Err(_) => {
            acc.push(0);
            acc
          },
        }
      })
      .into_iter()
      .max()
      .expect(ERROR_MESSAGE);
    println!("Answer: {:?}", answer);
  }
