extern crate bump_alloc;
use bump_alloc::BumpAlloc;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader, BufWriter};
use std::rc::Rc;
#[global_allocator]
static A: BumpAlloc = BumpAlloc::new();
#[derive(Eq, PartialEq, Debug)]
struct Player {
    name: String,
    skill1: u32,
    skill2: u32,
    skill3: u32,
    used: bool,
}
#[derive(Eq, PartialEq, Debug)]
struct PlayerKey {
    skill: u32,
    player: Rc<RefCell<Player>>,
}
impl PartialOrd for PlayerKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PlayerKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.skill > other.skill {
            Ordering::Greater
        } else if self.skill < other.skill {
            Ordering::Less
        } else {
            self.player.borrow().name.cmp(&other.player.borrow().name)
        }
    }
}
fn main() {
    let mut input = String::new();
    let mut stdin = BufReader::new(stdin());
    stdin.read_line(&mut input).unwrap();
    let mut stdout = BufWriter::new(stdout());
    let number_of_players = u32::from_str_radix(&input.trim(), 10).unwrap();
    let mut skill1_heap = BinaryHeap::new();
    let mut skill2_heap = BinaryHeap::new();
    let mut skill3_heap = BinaryHeap::new();
    input.truncate(0);
    while stdin.read_line(&mut input).unwrap() > 0 {
        let temp: Vec<&str> = input.trim().split(' ').collect();
        if temp.len() < 2 {
            break;
        }
        let skill1 = u32::from_str_radix(temp[1], 10).unwrap();
        let skill2 = u32::from_str_radix(temp[2], 10).unwrap();
        let skill3 = u32::from_str_radix(temp[3], 10).unwrap();
        let player = Rc::new(RefCell::new(Player {
            name: temp[0].to_string(),
            skill1: skill1,
            skill2: skill2,
            skill3: skill3,
            used: false,
        }));
        skill1_heap.push(PlayerKey {
            skill: skill1,
            player: Rc::clone(&player),
        });
        skill2_heap.push(PlayerKey {
            skill: skill2,
            player: Rc::clone(&player),
        });
        skill3_heap.push(PlayerKey {
            skill: skill3,
            player: Rc::clone(&player),
        });
        input.truncate(0);
    }
    let mut n = 1;
    while n < (number_of_players - number_of_players % 3) {
        while skill1_heap.peek().is_some() && skill1_heap.peek().unwrap().player.borrow().used {
            skill1_heap.pop();
        }
        let skill1_player = match skill1_heap.pop() {
            Some(e) => e,
            None => return,
        };
        skill1_player.player.borrow_mut().used = true;
        while skill2_heap.peek().is_some() && skill2_heap.peek().unwrap().player.borrow().used {
            skill2_heap.pop();
        }
        let skill2_player = match skill2_heap.pop() {
            Some(e) => e,
            None => return,
        };
        skill2_player.player.borrow_mut().used = true;
        while skill3_heap.peek().is_some() && skill3_heap.peek().unwrap().player.borrow().used {
            skill3_heap.pop();
        }
        let skill3_player = match skill3_heap.pop() {
            Some(e) => e,
            None => return,
        };
        skill3_player.player.borrow_mut().used = true;
        let mut out = vec![
            skill1_player.player.borrow().name.clone(),
            skill2_player.player.borrow().name.clone(),
            skill3_player.player.borrow().name.clone(),
        ];
        out.sort();
        stdout
            .write_fmt(format_args!("{} {} {}\n", out[0], out[1], out[2]))
            .unwrap();
        n = n + 1;
    }
}
