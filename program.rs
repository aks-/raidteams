use std::cmp::{Ordering, Reverse};
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader, BufWriter};
#[derive(Eq, PartialEq)]
struct Player {
    name: String,
    skill1: u32,
    skill2: u32,
    skill3: u32,
}
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
    input.truncate(0);
    let mut skill1_tree = Vec::with_capacity(number_of_players);
    let mut skill2_tree = Vec::with_capacity(number_of_players);
    let mut skill3_tree = Vec::with_capacity(number_of_players);
    let mut used: Vec<bool> = Vec::with_capacity(number_of_players);
    stdin.read_to_string(&mut input).unwrap();
    for line in input.as_str().split("\n") {
        let temp: Vec<&str> = line.trim().split(' ').collect();
        let idx = used.len();
        if temp.len() < 2 {
            break;
        }
        let name = temp[0];
        let skill1 = u32::from_str_radix(temp[1], 10).unwrap();
        let skill2 = u32::from_str_radix(temp[2], 10).unwrap();
        let skill3 = u32::from_str_radix(temp[3], 10).unwrap();
        used.push(false);
        skill1_tree.push(PlayerKey {
            skill: skill1,
            name: &name,
            idx: idx,
        });
        skill2_tree.push(PlayerKey {
            skill: skill2,
            name: &name,
            idx: idx,
        });
        skill3_tree.push(PlayerKey {
            skill: skill3,
            name: &name,
            idx: idx,
        });
    }
    skill1_tree.sort();
    skill2_tree.sort();
    skill3_tree.sort();
    loop {
        while skill1_tree.last().is_some() && used[skill1_tree.last().unwrap().idx] {
            skill1_tree.pop();
        }
        let skill1_player = match skill1_tree.pop() {
            Some(e) => e,
            None => return,
        };
        used[skill1_player.idx] = true;
        while skill2_tree.last().is_some() && used[skill2_tree.last().unwrap().idx] {
            skill2_tree.pop();
        }
        let skill2_player = match skill2_tree.pop() {
            Some(e) => e,
            None => return,
        };
        used[skill2_player.idx] = true;
        while skill3_tree.last().is_some() && used[skill3_tree.last().unwrap().idx] {
            skill3_tree.pop();
        }
        let skill3_player = match skill3_tree.pop() {
            Some(e) => e,
            None => return,
        };
        used[skill3_player.idx] = true;
        let mut out = vec![skill1_player.name, skill2_player.name, skill3_player.name];
        out.sort();
        stdout
            .write_fmt(format_args!("{} {} {}\n", out[0], out[1], out[2]))
            .unwrap();
    }
}
