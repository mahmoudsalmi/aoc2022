use std::{collections::HashMap, io::Result};

const YEAR: &'static str = "2022";
const DAY: &'static str = "07";

#[derive(Debug)]
struct Node {
    name: String,
    size: u64,
    dir: bool,
    parent: Option<String>,
    childs: Option<Vec<String>>,
}

impl Node {
    fn root() -> Node {
        Node {
            name: "/".to_owned(),
            size: 0,
            dir: true,
            parent: None,
            childs: Some(vec![]),
        }
    }

    fn new_dir(name: String, parent: String) -> Node {
        Node {
            name,
            size: 0,
            dir: true,
            parent: Some(parent),
            childs: Some(vec![]),
        }
    }

    fn new_file(name: String, parent: String, size: u64) -> Node {
        Node {
            name,
            size,
            dir: false,
            parent: Some(parent),
            childs: None,
        }
    }

    fn idx(&self, nodes: &HashMap<String, Node>) -> String {
        if self.parent.is_none() {
            return "/".to_owned();
        }

        let parent: &Node = self
            .parent
            .clone()
            .map(|name| nodes.get(&name))
            .flatten()
            .unwrap();

        let res = [parent.idx(nodes), self.name.clone()]
            .map(|i| if i == "/" { "".to_owned() } else { i })
            .join("/");
        res
    }

    fn add_child(&mut self, child_name: String) {
        if self.childs.is_some() {
            let childs: &mut Vec<String> = self.childs.as_mut().unwrap();
            childs.push(child_name);
        } else {
            unreachable!("NO CHILDS!!");
        }
    }
}

#[derive(Debug)]
struct Tree {
    current: String,
    root: String,
    indexes: HashMap<String, Node>,
}

impl Tree {
    fn new() -> Tree {
        let root: Node = Node::root();
        let root_idx: String = root.name.clone();

        Tree {
            indexes: HashMap::from([(root_idx.clone(), root)]),
            current: root_idx.clone(),
            root: root_idx.clone(),
        }
    }

    fn add_file(&mut self, name: String, size: u64) {
        let nodes = &mut (self.indexes);

        let curr_name: String = name;
        let mut parent_name: Option<String> = Some(self.current.clone());

        let node = Node::new_file(
            curr_name.clone(),
            parent_name.clone().unwrap().clone(),
            size,
        );

        let node_idx: String = node.idx(nodes);

        nodes.insert(node.idx(nodes), node);

        let mut parent: Option<&mut Node> = parent_name
            .clone()
            .map(|name| nodes.get_mut(&name.clone()))
            .flatten();

        parent
            .iter_mut()
            .for_each(|p| p.add_child(node_idx.clone()));

        while !parent.is_none() {
            let parent_node = parent.unwrap();
            parent_node.size += size;

            parent_name = parent_node.parent.clone();

            parent = parent_name
                .clone()
                .map(|name| nodes.get_mut(&name.clone()))
                .flatten();
        }
    }

    fn add_dir(&mut self, name: String) {
        let nodes = &mut (self.indexes);

        let parent_name: String = self.current.clone();

        let node: Node = Node::new_dir(name, parent_name.clone());
        let child_idx: String = node.idx(nodes);

        nodes
            .get_mut(&parent_name.clone())
            .iter_mut()
            .for_each(|p| p.add_child(child_idx.clone()));

        nodes.insert(node.idx(nodes), node);
    }

    fn get_node(&self, node_idx: String) -> Option<&Node> {
        self.indexes.get(&node_idx)
    }

    fn get_root_node(&self) -> Option<&Node> {
        self.get_node(self.root.clone())
    }

    fn goto(&mut self, sub_dir: String) {
        if sub_dir == self.root {
            self.current = sub_dir;
            return;
        }

        let current_node = self
            .indexes
            .get(&self.current.clone())
            .expect("Node not found!");

        self.current = if sub_dir == ".." {
            self.indexes
                .get(&current_node.parent.clone().expect("No parent name found!"))
                .expect("Parent not found!")
                .idx(&self.indexes)
        } else {
            let child_idx = current_node
                .childs
                .as_ref()
                .expect("No childs")
                .iter()
                .map(|child| self.indexes.get(child).expect("No child Node"))
                .find(|child| child.name == sub_dir)
                .expect("No child!")
                .idx(&self.indexes);

            self.indexes
                .get(&child_idx)
                .expect("No child node!")
                .idx(&self.indexes)
        };
    }
}

fn parse_tree(data: &str) -> Tree {
    let mut tree = Tree::new();

    data.lines().for_each(|line| match line {
        cd_command if cd_command.starts_with("$ cd") => {
            let dest: String = cd_command
                .split_whitespace()
                .skip(2)
                .take(1)
                .collect::<String>();
            tree.goto(dest);
        }
        ls_command if ls_command.starts_with("$ ls") => {}
        dir_line if dir_line.starts_with("dir") => {
            let dir_name = dir_line
                .split_whitespace()
                .skip(1)
                .take(1)
                .collect::<String>();
            tree.add_dir(dir_name);
        }
        file_line => {
            let filename = file_line
                .split_whitespace()
                .skip(1)
                .take(1)
                .collect::<String>();
            let size = file_line
                .split_whitespace()
                .take(1)
                .collect::<String>()
                .parse::<u64>()
                .expect("Invalid size!");
            tree.add_file(filename, size);
        }
    });
    tree
}

fn part1(data: &str) -> u64 {
    let tree = parse_tree(data);
    tree.indexes
        .values()
        .filter(|node| node.dir && node.size <= 100000)
        .map(|node| node.size)
        .sum()
}

fn part2(data: &str) -> u64 {
    let tree = parse_tree(data);
    let remain_space: u64 = 70000000 - tree.get_root_node().unwrap().size;
    let space_needed: u64 = 30000000 - remain_space;

    let mut dirs: Vec<&Node> = tree.indexes.values().filter(|node| node.dir && node.size >= space_needed).collect();
    dirs.sort_by(|a, b| a.size.cmp(&b.size));
    dirs.first().unwrap().size
}

fn main() -> Result<()> {
    println!();
    println!(
        "---( AOC{} - Day {} )-----------------------[Rust]----",
        YEAR, DAY
    );

    let test_data = include_str!("day07.test");
    println!("Test :: Part 1 ====>     {}", part1(&test_data));
    println!("Test :: Part 2 ====>     {}", part2(&test_data));
    println!("--------------------------------------------------------");

    let input_data = include_str!("day07.in");
    println!("Input:: Part 1 ====>     {}", part1(&input_data));
    println!("Input:: Part 2 ====>     {}", part2(&input_data));
    println!("--------------------------------------------------------");

    Ok(())
}
