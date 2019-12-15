use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidOrbit(String),
}

struct Orbit {
    pub center: String,
    pub object: String,
}

impl FromStr for Orbit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(')').collect();
        if parts.len() != 2 {
            Err(Error::InvalidOrbit(s.to_string()))
        } else {
            Ok(Orbit {
                center: parts[0].to_string(),
                object: parts[1].to_string(),
            })
        }
    }
}

fn count_orbits_1(obj: &str, data: &HashMap<&str, &str>) -> u64 {
    let v = data.get(obj).unwrap();
    if v == &"COM" {
        1
    } else {
        1 + count_orbits_1(v, data)
    }
}

fn count_orbits(orbits: &Vec<Orbit>) -> u64 {
    let mut map: HashMap<&str, &str> = HashMap::new();
    for o in orbits.iter() {
        map.insert(&o.object, &o.center);
    }
    orbits.iter().map(|o| count_orbits_1(&o.object, &map)).sum()
}

#[derive(Clone, Debug)]
struct Node {
    pub id: usize,
    pub name: String,
    pub children: Vec<usize>,
}

struct Arena {
    pub nodes: Vec<Rc<RefCell<Node>>>,
}

impl Arena {
    fn new() -> Arena {
        Arena { nodes: Vec::new() }
    }

    fn new_node(&mut self, name: &String) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node {
            id: self.nodes.len(),
            name: name.clone(),
            children: Vec::new(),
        }));
        self.nodes.push(node.clone());
        node
    }

    fn get_node(&self, id: usize) -> Rc<RefCell<Node>> {
        self.nodes[id].clone()
    }
}

fn make_orbit_graph(orbits: &Vec<Orbit>) -> (Arena, HashMap<String, Rc<RefCell<Node>>>) {
    let mut arena = Arena::new();
    let mut graph = HashMap::new();
    for o in orbits.iter() {
        let left_id = graph
            .entry(o.center.clone())
            .or_insert_with(|| arena.new_node(&o.center))
            .borrow()
            .id;
        let right_id = graph
            .entry(o.object.clone())
            .or_insert_with(|| arena.new_node(&o.object))
            .borrow()
            .id;
        graph
            .get_mut(&o.center)
            .unwrap()
            .borrow_mut()
            .children
            .push(right_id);
        graph
            .get_mut(&o.object)
            .unwrap()
            .borrow_mut()
            .children
            .push(left_id);
    }
    (arena, graph)
}

fn min_transfers(orbits: &Vec<Orbit>, source: &str, dest: &str) -> u64 {
    let (arena, graph) = make_orbit_graph(orbits);
    let mut queue: VecDeque<(Rc<RefCell<Node>>, u64)> = VecDeque::new();
    queue.push_front((graph.get(&source.to_string()).unwrap().clone(), 0));
    let mut seen: HashMap<String, u64> = HashMap::new();
    seen.insert(source.to_string(), 0);

    let dest = dest.to_string();
    loop {
        let (node, depth) = queue.pop_back().unwrap();
        for child_id in node.borrow().children.iter() {
            let child = arena.get_node(*child_id).clone();
            if !seen.contains_key(&child.borrow().name) {
                seen.insert(child.borrow().name.clone(), depth + 1);
                queue.push_front((child, depth + 1));
            }
        }
        if seen.contains_key(&dest) {
            break;
        }
    }
    seen.get(&dest).unwrap() - 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(env::args().nth(1).unwrap())?;
    let orbits: Vec<Orbit> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse())
        .map(Result::unwrap)
        .collect();
    let count = count_orbits(&orbits);
    println!("Total number of orbits: {}", count);
    println!("Distance: {}", min_transfers(&orbits, "YOU", "SAN"));
    Ok(())
}
