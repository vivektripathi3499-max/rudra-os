extern crate alloc;

use crate::println;
use alloc::vec;  
use alloc::vec::Vec;

#[derive(Clone)]
pub enum NodeType {
    File,
    Folder,
}

#[derive(Clone)]
pub struct Node {
    pub name: &'static str,
    pub node_type: NodeType,
    pub children: Vec<Node>,
    pub content: Option<&'static str>,
}

/* =========================
   SAMPLE FILESYSTEM
========================= */

pub fn root_fs() -> Node {
    Node {
        name: "/",
        node_type: NodeType::Folder,
        content: None,
        children: vec![
            Node {
                name: "docs",
                node_type: NodeType::Folder,
                content: None,
                children: vec![
                    Node {
                        name: "readme.txt",
                        node_type: NodeType::File,
                        content: Some("Hello from Rudra OS"),
                        children: vec![],
                    },
                ],
            },
            Node {
                name: "hello.txt",
                node_type: NodeType::File,
                content: Some("Simple file"),
                children: vec![],
            },
        ],
    }
}

/* =========================
   PATH NAVIGATION
========================= */

pub fn get_node_by_path<'a>(root: &'a Node, path: &str) -> &'a Node {
    let mut current = root;

    for part in path.split("/").filter(|p| !p.is_empty()) {
        current = current
            .children
            .iter()
            .find(|c| c.name == part)
            .expect("Path not found");
    }

    current
}
