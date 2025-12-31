use bevy::prelude::*;

use crate::node::INode;

#[derive(Debug, Component, Default, Clone)]
#[require(INode::new_file())]
pub struct File {
    pub content: Vec<u8>,
}
impl File {
    pub fn get_size(&self) -> u64 {
        self.content.len() as u64
    }
    pub fn truncate(&mut self, new_len: u64) {
        if new_len <= self.content.len() as u64 {
            self.content.truncate(new_len.try_into().unwrap());
        } else {
            for _ in self.content.len() as u64..new_len {
                self.content.push(0);
            }
        }
    }
}
