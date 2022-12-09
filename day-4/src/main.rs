use std::fs;
use regex::Regex;
use itertools::Itertools;


const ERROR_MESSAGE: &str = "Some weird error happened. Help!";

fn main() {
    let file = fs::read_to_string("input").expect(ERROR_MESSAGE);
    let re = Regex::new(r"\d+").unwrap();
    let answer = file
      .lines()
      .fold(0, |acc, line| {
        let (s1, e1, s2, e2) = re
          .find_iter(line)
          .map(|m| 
            line
              .get(m.start()..m.end())
              .expect(ERROR_MESSAGE)
              .parse::<u32>()
              .unwrap()
          )
          .collect_tuple()
          .unwrap();

        let contained_1 = s1 <= s2 && e1 >= e2;
        let contained_2 = s2 <= s1 && e2 >= e1;

        if contained_1 || contained_2 {
          acc + 1
        } else {
          acc
        }
      });
    println!("Answer: {:?}", answer);
  }
