use crate::solution::Solution;
use maplit::hashset;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[cfg(test)]
mod test;
pub struct Day23 {}

#[derive(Clone, PartialEq, Eq)]
struct Computer {
    name: String,
    neighbors: RefCell<Vec<Rc<Computer>>>,
}

impl Computer {
    fn new(name: String) -> Rc<Self> {
        Rc::new(Self {
            name,
            neighbors: RefCell::new(Vec::new()),
        })
    }

    fn add_neighbor(&self, computer: Rc<Computer>) {
        self.neighbors.borrow_mut().push(computer);
    }
}

impl Hash for Computer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

struct LAN {
    computers: Vec<Rc<Computer>>,
}

fn sort_computers(mut computers: Vec<Rc<Computer>>) -> Vec<Rc<Computer>> {
    computers.sort_by(|x, y| x.name.cmp(&y.name));
    computers
}

type ComputerSet = HashSet<Rc<Computer>>;

impl LAN {
    fn walk(
        &self,
        computer: &Rc<Computer>,
        steps: u32,
        breadcrumbs: Vec<Rc<Computer>>,
    ) -> HashSet<Vec<Rc<Computer>>> {
        let neighbors: Vec<Rc<Computer>> = {
            let neighbors_borrowed = computer.neighbors.borrow();
            neighbors_borrowed
                .iter()
                .filter(|&neighbor| !breadcrumbs.contains(neighbor))
                .cloned()
                .collect()
        };

        let mut new_breadcrumbs = breadcrumbs.clone();
        new_breadcrumbs.push(computer.clone());

        if steps == 0 {
            return if computer
                .neighbors
                .borrow()
                .contains(new_breadcrumbs.first().unwrap())
            {
                hashset! { sort_computers(new_breadcrumbs) }
            } else {
                hashset! {}
            };
        }

        let mut cycles = HashSet::new();

        for neighbor in neighbors {
            cycles.extend(self.walk(&neighbor.clone(), steps - 1, new_breadcrumbs.clone()));
        }

        cycles
    }

    fn find_cycles(&self, length: u32) -> HashSet<Vec<Rc<Computer>>> {
        let mut cycles = HashSet::new();
        for node in self.computers.iter() {
            cycles.extend(self.walk(node, length - 1, vec![]))
        }

        cycles
    }

    fn bron_kerbosch(&self, r: ComputerSet, p: ComputerSet, x: ComputerSet) -> Vec<ComputerSet> {
        if p.is_empty() && x.is_empty() {
            return vec![r];
        }

        let mut cliques: Vec<ComputerSet> = Vec::new();

        let mut current_p = p.clone();
        let mut current_x = x;

        for v in p {
            let neighbors: ComputerSet = v.neighbors.borrow().clone().into_iter().collect();
            cliques.extend(self.bron_kerbosch(
                r.union(&hashset! { v.clone() }).cloned().collect(),
                current_p.intersection(&neighbors).cloned().collect(),
                current_x.intersection(&neighbors).cloned().collect(),
            ));

            current_p.remove(&v.clone());
            current_x.insert(v);
        }

        cliques
    }

    fn find_maximum_clique(&self) -> Vec<Rc<Computer>> {
        let cliques = self.bron_kerbosch(
            hashset!(),
            self.computers.iter().cloned().collect::<ComputerSet>(),
            hashset!(),
        );

        let maximum_clique = cliques
            .iter()
            .max_by_key(|c| c.len())
            .unwrap()
            .iter()
            .collect::<Vec<_>>();

        maximum_clique.iter().map(|&c| c.clone()).collect()
    }
}

fn parse_input(input: &str) -> LAN {
    let mut computers: HashMap<String, Rc<Computer>> = HashMap::new();

    let string_connections: Vec<(String, String)> = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(|l| l.split_once('-').unwrap())
        .map(|(l, r)| (l.to_string(), r.to_string()))
        .collect::<Vec<(String, String)>>();

    for (left, right) in &string_connections {
        computers
            .entry(left.clone())
            .or_insert_with(|| Computer::new(left.clone()));
        computers
            .entry(right.clone())
            .or_insert_with(|| Computer::new(right.clone()));
    }

    string_connections
        .into_iter()
        .for_each(|(left, right)| {
            let left = computers.get(&left).unwrap();
            let right = computers.get(&right).unwrap();
            left.add_neighbor(right.clone());
            right.add_neighbor(left.clone());
        });

    LAN {
        computers: computers.values().cloned().collect(),
    }
}

impl fmt::Display for LAN {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(
            self.computers
                .iter()
                .map(|c| c.name.clone())
                .collect::<Vec<String>>()
                .join(", ")
                .as_str(),
        )
    }
}

impl Solution for Day23 {
    fn solve_part1(&self, input: &str) -> String {
        let lan = parse_input(input);

        let triples = lan.find_cycles(3);
        triples
            .iter()
            .filter(|l| l.iter().any(|c| c.name.starts_with("t")))
            .count()
            .to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let lan = parse_input(input);

        let maximum_clique = lan.find_maximum_clique();
        let sorted = sort_computers(maximum_clique);

        format!("{}", sorted.iter().map(|c| c.name.clone()).collect::<Vec<String>>().join(","))
    }
}
