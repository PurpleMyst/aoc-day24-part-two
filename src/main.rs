#![feature(fs_read_write)]
use std::{fs, mem};

extern crate rpds;
use rpds::HashTrieSet;

mod component;
use component::Component;

mod bridge;
use bridge::Bridge;

fn main() {
    let components = fs::read_string("input.txt")
        .unwrap()
        .lines()
        .map(&str::parse)
        .collect::<Result<HashTrieSet<Component>, _>>()
        .unwrap();

    let mut bridges = vec![Bridge::new(components)];
    let mut best_bridge_strength = 0;

    while !bridges.is_empty() {
        // assert we're not using more than a megabyte of memory
        debug_assert!(mem::size_of_val(&bridges) <= 1 * 1024 * 1024);

        bridges = bridges.into_iter().flat_map(|bridge| {
            let (children, bridge) = bridge.children();

            if children.is_empty() {
                if bridge.strength > best_bridge_strength {
                    best_bridge_strength = bridge.strength;
                }

                vec![]
            } else {
                children
            }
        }).collect();
    }

    println!("{}", best_bridge_strength);
}
