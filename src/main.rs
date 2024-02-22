use std::path::PathBuf;
use serde_yaml::mapping::Iter;

use crate::jewel::Jewel;

pub mod path;
pub mod fs;

pub mod shard;
pub mod jewel;


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


fn main() {
    let jewel = Jewel::open("C:\\Users\\gael.pabois\\Documents\\PADVME\\PADVME").unwrap();

    for entry in jewel.walk_shards() {
      println!("{:?}", entry.ast);
      return;
    }

}
