use std::collections::HashSet;

use super::component::Component;

#[derive(Debug)]
pub struct Bridge<'a> {
    components: HashSet<&'a Component>,
    last_port: usize,
}

impl<'a> Bridge<'a> {
    pub fn new() -> Self {
        Self {
            components: HashSet::new(),
            last_port: 0,
        }
    }

    pub fn successors(&self, components: &'a Vec<Component>) -> Vec<Self> {
        components
            .iter()
            .filter(|component| !self.components.contains(component))
            .filter_map(|component| {
                let new_last = if component.left == self.last_port {
                    component.right
                } else if component.right == self.last_port {
                    component.left
                } else {
                    return None;
                };

                let mut new_components = self.components.clone();
                new_components.insert(component);

                Some(Self {
                    components: new_components,
                    last_port: new_last,
                })
            })
            .collect()
    }

    pub fn strength(&self) -> usize {
        self.components
            .iter()
            .map(|component| component.left + component.right)
            .sum()
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }
}
