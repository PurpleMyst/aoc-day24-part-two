use std::rc::Rc;

use super::component::{Component, Components};

#[derive(Debug)]
pub struct Bridge {
    pub available_port: u8,
    pub previous_component: Option<Rc<Bridge>>,

    pub strength: usize,
    pub length: usize,

    available_components: Components,
}

impl Bridge {
    pub fn new(components: Components) -> Self {
        Self {
            available_port: 0,
            previous_component: None,

            strength: 0,
            length: 0,

            available_components: components,
        }
    }

    pub fn add_component(self_rc: Rc<Self>, component: Component, available_port: u8) -> Self {
        Self {
            available_port,

            strength: self_rc.strength + (component.left as usize) + (component.right as usize),
            length: self_rc.length + 1,

            available_components: self_rc.available_components.remove(&component),

            // after everything cause this moves
            previous_component: Some(self_rc),
        }
    }

    // TODO: We could use `impl Trait` here, maybe?
    pub fn children(self) -> (Vec<Self>, Rc<Self>) {
        let self_rc = Rc::new(self);

        (self_rc.available_components
            .iter()
            .filter_map(|component: &Component| -> Option<Self> {
                if component.left == self_rc.available_port {
                    Some(Self::add_component(Rc::clone(&self_rc), *component, component.right))
                } else if component.right == self_rc.available_port {
                    Some(Self::add_component(Rc::clone(&self_rc), *component, component.left))
                } else {
                    None
                }
            })
            .collect(), self_rc)
    }
}
