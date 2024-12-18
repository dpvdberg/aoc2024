use crate::solution::Solution;
use crate::utils::geometry::Direction;
use crate::utils::nalgebra::{MatrixHelpers, MatrixParser, VectorHelpers};
use nalgebra::{DMatrix, Vector2};
use priority_queue::PriorityQueue;
use std::cmp::{min, Reverse};
use std::collections::{HashMap, HashSet, VecDeque};
use strum::IntoEnumIterator;

#[cfg(test)]
mod test;
pub struct Day16 {}

struct Maze {
    tiles: DMatrix<MazeTile>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct MazeNode {
    position: Vector2<i32>,
    direction: Direction,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum MazeTile {
    Start,
    End,
    Wall,
    Floor,
}

impl Maze {
    fn walk_to_neighbors(&self, node: &MazeNode) -> HashMap<MazeNode, u32> {
        let mut position = node.position + node.direction.to_vector();
        let mut neighbors = HashMap::new();
        let mut steps = 1;

        while self.tiles.at_value(&position) != MazeTile::Wall {
            let sides = Direction::iter()
                .filter(|d| node.direction.is_vertical() != d.is_vertical())
                .map(|d| position + d.to_vector())
                .filter(|p| self.tiles.at_value(p) != MazeTile::Wall)
                .count();

            if sides > 0 {
                neighbors.insert(
                    MazeNode {
                        position,
                        direction: node.direction.clone(),
                    },
                    steps,
                );
            }

            position = position + node.direction.to_vector();
            steps += 1;
        }

        neighbors
    }

    fn find_neighbors(&self, node: &MazeNode) -> HashMap<MazeNode, u32> {
        let mut neighbors: HashMap<MazeNode, u32> = HashMap::new();

        let forward_neighbors = self.walk_to_neighbors(&node);
        neighbors.extend(forward_neighbors);

        let right_neighbors = self.walk_to_neighbors(&MazeNode {
            position: node.position,
            direction: node.direction.clockwise(),
        });
        neighbors.extend(
            right_neighbors
                .into_iter()
                .map(|(k, v)| (k, v + 1000))
                .collect::<HashMap<_, _>>(),
        );

        let left_neighbors = self.walk_to_neighbors(&MazeNode {
            position: node.position,
            direction: node.direction.counter_clockwise(),
        });
        neighbors.extend(
            left_neighbors
                .into_iter()
                .map(|(k, v)| (k, v + 1000))
                .collect::<HashMap<_, _>>(),
        );

        neighbors
    }

    fn shortest_path(&self) -> (u32, HashMap<MazeNode, Vec<MazeNode>>, MazeNode) {
        let start = self.tiles.find_index(MazeTile::Start).unwrap();
        let end = self.tiles.find_index(MazeTile::End).unwrap();

        let mut distances: HashMap<MazeNode, u32> = HashMap::new();
        let mut pred: HashMap<MazeNode, Vec<MazeNode>> = HashMap::new();

        let mut queue: PriorityQueue<MazeNode, _> = PriorityQueue::new();
        queue.push(
            MazeNode {
                position: start,
                direction: Direction::Right,
            },
            Reverse(0),
        );

        while !queue.is_empty() {
            let (node, distance) = queue.pop().unwrap();
            let distance = distance.0;

            distances.insert(node.clone(), distance);

            if node.position == end {
                break;
            }

            let neighbors = self.find_neighbors(&node);
            for (neighbor, neighbor_distance) in neighbors {
                if distances.contains_key(&neighbor) {
                    continue;
                }

                if queue.iter().any(|f| *f.0 == neighbor) {
                    let current_distance = queue.get_priority(&neighbor).unwrap().0;
                    let new_distance = distance + neighbor_distance;
                    if new_distance < current_distance {
                        pred.insert(neighbor.clone(), vec![node.clone()]);
                    } else if new_distance == current_distance {
                        pred.get_mut(&neighbor.clone()).unwrap().push(node.clone());
                    }

                    let new_priority = min(current_distance, new_distance);
                    // println!("Updating priority of {:?} (direction: {:?}) from {} to {}", neighbor.position, neighbor.direction, current_distance, new_priority);
                    queue.change_priority(&neighbor, Reverse(new_priority));
                } else {
                    // println!("Adding {:?} (direction: {:?}) at distance {:?}", neighbor.position, neighbor.direction, distance + neighbor_distance);
                    pred.insert(neighbor.clone(), vec![node.clone()]);
                    queue.push(neighbor, Reverse(distance + neighbor_distance));
                }
            }
        }

        let (end_node, distance) = distances
            .iter()
            .filter(|(n, _)| self.tiles.at_value(&n.position) == MazeTile::End)
            .min_by_key(|(_, &d)| d)
            .unwrap();

        (*distance, pred, end_node.clone())
    }

    fn shortest_path_length(&self) -> u32 {
        self.shortest_path().0
    }

    fn observer_wall_count(&self) -> u32 {
        let (_, predecessors, end_node) = self.shortest_path();

        let mut best_path_points: HashSet<Vector2<i32>> = HashSet::new();

        let mut visited: HashSet<Vector2<i32>> = HashSet::new();
        let mut remaining: VecDeque<MazeNode> = VecDeque::new();
        remaining.push_front(end_node.clone());

        while !remaining.is_empty() {
            let current = remaining.pop_front().unwrap();
            visited.insert(current.position);

            if !predecessors.contains_key(&current) {
                continue;
            }

            if let Some(predecessors) = predecessors.get(&current) {
                // println!("predecessors of {:?}: {:?}", current.position, predecessors.iter().map(|e| e.position).collect::<Vec<_>>());

                for predecessor in predecessors {
                    if visited.contains(&predecessor.position) {
                        continue;
                    }
                    remaining.push_back(predecessor.clone());

                    best_path_points.extend(
                        current
                            .position
                            .get_intermediate_points(&predecessor.position),
                    );
                }
            }
        }

        best_path_points.iter().count() as u32
    }
}

fn parse_input(input: &str) -> Maze {
    Maze {
        tiles: input.to_string().to_matrix(|c| match c {
            'S' => MazeTile::Start,
            'E' => MazeTile::End,
            '#' => MazeTile::Wall,
            '.' => MazeTile::Floor,
            _ => panic!("Unexpected character in input: {}", c),
        }),
    }
}

impl Solution for Day16 {
    fn solve_part1(&self, input: &str) -> String {
        let maze = parse_input(input);
        maze.shortest_path_length().to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let maze = parse_input(input);
        maze.observer_wall_count().to_string()
    }
}
