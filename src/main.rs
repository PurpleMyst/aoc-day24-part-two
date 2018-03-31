#![feature(fs_read_write)]
use std::fs;

mod component;
use component::Component;

mod bridge;
use bridge::Bridge;

fn main() {
    let components = fs::read_string("input.txt")
        .unwrap()
        .lines()
        .map(&str::parse)
        .collect::<Result<Vec<Component>, _>>()
        .unwrap();

    let mut bridges = vec![Bridge::new()];
    let mut best_bridge: Option<(usize, usize)> = None;

    while !bridges.is_empty() {
        debug_assert!(bridges.len() < 1_000_000);

        bridges = bridges
            .into_iter()
            .filter_map(|bridge| {
                let successors = bridge.successors(&components);

                if successors.is_empty() {
                    let this_bridge_is_better =
                        best_bridge
                            .map(|(length, strength)| {
                                if bridge.len() > length {
                                    true
                                } else if bridge.len() == length {
                                    bridge.strength() > strength
                                } else {
                                    false
                                }
                            })
                            .unwrap_or(true);

                    if this_bridge_is_better {
                        best_bridge = Some((bridge.len(), bridge.strength()));
                    }

                    None
                } else {
                    Some(successors)
                }
            })
            .flat_map(|v| v) // XXX: Is there a `flatten` option?
            .collect()
    }

    println!("{}", best_bridge.unwrap().1);
}
