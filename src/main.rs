#![feature(fs_read_write)]
use std::{fs, mem};

extern crate rpds;

mod component;
use component::Components;

mod bridge;
use bridge::Bridge;

fn main() {
    let components = fs::read_string("input.txt")
        .unwrap()
        .lines()
        .map(&str::parse)
        .collect::<Result<Components, _>>()
        .unwrap();

    let mut bridges = vec![Bridge::new(components)];
    let mut best_bridge_strength = 0;
    let mut best_bridge_length = 0;

    while !bridges.is_empty() {
        // assert we're not using more than a megabyte of memory
        debug_assert!(mem::size_of_val(&bridges) <= 1 * 1024 * 1024);

        bridges = bridges.into_iter().flat_map(|bridge| {
            let (children, bridge) = bridge.children();

            if children.is_empty() {
                if bridge.strength >= best_bridge_strength || bridge.length > best_bridge_length {
                    best_bridge_strength = bridge.strength;
                    best_bridge_length = bridge.length;
                }

                vec![]
            } else {
                children
            }
        }).collect();
    }

    println!("{}", best_bridge_strength);
}
