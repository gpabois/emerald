pub mod shard;

const INPUT: &str = r#"---
title: gray-matter-rs
tags:
  - gray-matter
  - rust
---
Some excerpt
---
Other stuff
"#;


pub struct Jewel {
    root: String
}

pub struct WalkShard<'a> {
    root: &'a Jewel
}

fn main() {
    
    let shard = shard::Shard::from_str(INPUT);
    println!("{:?}", shard);
}
