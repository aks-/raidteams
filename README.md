### RaidTeams
Raid Teams is a problem on https://open.kattis.com/problems/raidteams. I was challenged to beat 0.11 sec by someone. I decided to go with Rust because using that I could micromanage memory allocation.


### How to install Rust
- I installed `rustup` with `brew`, then `rustup init` installed `rust` and `cargo`.
- Then I installed `flamegraph` by running `cargo install flamegraph`

### How to use flamegraph
`cargo flaemgraph < inputfile`

### Steps I took to make it faster so that runtime is 0.8 sec
- First attempt: binary tree, the comparision was slower
- Second attempt: binary heap, but it is slow — most of the time was in binaryheap pop, which was calling cmp a bunch.
- Tried btreeset — similar, but most of the time was in btreeset insert, which called cmp a lot. 0.25s
- Then switched to vectors — inserting is fast, at the end, so did that.  Has to sort it. Got to 0.15.
- Reworked it to eliminate players object, only storing (skill, name) in 3 arrays, and an array of booleans for if they got used yet. 0.13s.
- Calling sort on a vector is sorting; it uses Ord to compare. But sorting an array is more optimized it turns out. Then the last bits are hacking away overhead. Iterator allocating memory as it goes. Splitting and joining strings both cost.
- Then Switched out format for a bunch of write statements.
- Replaced `sort` with `sort_unsafe` on vector to make it to 0.8 sec.

### Results
![Result image](/katis-rank-ashok.jpeg?raw=true "Katis Rank")
