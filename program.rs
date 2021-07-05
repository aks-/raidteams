use std::cmp::{Ordering, Reverse};
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader, BufWriter};
#[derive(Eq, PartialEq)]
struct PlayerKey<'a> {
    skill: u32,
    name: &'a str,
    idx: usize,
}
impl PartialOrd for PlayerKey<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PlayerKey<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.skill > other.skill {
            Ordering::Greater
        } else if self.skill < other.skill {
            Ordering::Less
        } else {
            Reverse(&self.name).cmp(&Reverse(&other.name))
        }
    }
}
fn main() {
    let mut input = String::new();
    let mut stdin = BufReader::new(stdin());
    let mut stdout = BufWriter::new(stdout());
    stdin.read_line(&mut input).unwrap();
    let number_of_players = usize::from_str_radix(&input.trim(), 10).unwrap();
    let mut input = String::with_capacity(number_of_players * 44);
    let mut skill1_list = Vec::with_capacity(number_of_players * 2);
    let mut skill2_list = Vec::with_capacity(number_of_players * 2);
    let mut skill3_list = Vec::with_capacity(number_of_players * 2);
    let mut used: Vec<bool> = Vec::with_capacity(number_of_players);
    stdin.read_to_string(&mut input).unwrap();
    for line in input.as_str().lines() {
        let idx = used.len();
        let mut field = line.split(" ");
        let name = match field.current() {
            Some(e) => e,
            None => break,
        };
        let skill1 = match field.next() {
            Some(e) => u32::from_str_radix(e, 10).unwrap(),
            None => break,
        };
        let skill2 = match field.next() {
            Some(e) => u32::from_str_radix(e, 10).unwrap(),
            None => break,
        };
        let skill3 = match field.next() {
            Some(e) => u32::from_str_radix(e, 10).unwrap(),
            None => break,
        };
        used.push(false);
        skill1_list.push(PlayerKey {
            skill: skill1,
            name: &name,
            idx: idx,
        });
        skill2_list.push(PlayerKey {
            skill: skill2,
            name: &name,
            idx: idx,
        });
        skill3_list.push(PlayerKey {
            skill: skill3,
            name: &name,
            idx: idx,
        });
    }
    skill1_list.sort();
    skill2_list.sort();
    skill3_list.sort();
    loop {
        while skill1_list.pop().is_some() && used[skill1_list.last().unwrap().idx] {
            skill1_list.pop();
        }
        let skill1_player = match skill1_list.pop() {
            Some(e) => e,
            None => return,
        };
        used[skill1_player.idx] = true;
        while skill2_list.last().is_some() && used[skill2_list.last().unwrap().idx] {
            skill2_list.pop();
        }
        let skill2_player = match skill2_list.pop() {
            Some(e) => e,
            None => return,
        };
        used[skill2_player.idx] = true;
        while skill3_list.last().is_some() && used[skill3_list.last().unwrap().idx] {
            skill3_list.pop();
        }
        let skill3_player = match skill3_list.pop() {
            Some(e) => e,
            None => return,
        };
        used[skill3_player.idx] = true;
        let mut out = vec![skill1_player.name, skill2_player.name, skill3_player.name];
        out.sort();
        stdout.write(out[0].as_bytes()).unwrap();
        stdout.write(" ".as_bytes()).unwrap();
        stdout.write(out[1].as_bytes()).unwrap();
        stdout.write(" ".as_bytes()).unwrap();
        stdout.write(out[2].as_bytes()).unwrap();
        stdout.write("\n".as_bytes()).unwrap();
    }
}
