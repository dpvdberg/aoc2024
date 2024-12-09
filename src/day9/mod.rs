use std::cmp::min;
use crate::solution::Solution;
use std::fmt;
pub use std::fmt::Write;

#[cfg(test)]
mod test;
pub struct Day9 {}

#[derive(Clone)]
struct Block {
    file_id: u32,
    size: usize,
    index: u32,
}

impl Block {
    fn checksum(&self) -> u64 {
        let indices_sum: u64 = (self.index..(self.index + self.size as u32))
            .map(|i| i as u64)
            .sum();

        indices_sum * self.file_id as u64
    }
}

#[derive(Clone)]
struct Disk {
    blocks: Vec<Block>,
}
impl Disk {
    fn insert_empty_block(&mut self, required_size: usize, max_index: usize) -> Option<(&mut Block, usize)> {
        let max_index = min(max_index, self.blocks.len() - 1);
        
        for i in 0..max_index {
            let block = &self.blocks[i];
            let block_ahead = &self.blocks[i+1];
            
            let empty_space = (block_ahead.index - (block.index + block.size as u32)) as usize;
            if empty_space >= required_size {
                let new_block = Block {
                    file_id: 0,
                    size: 0,
                    index: block.index + block.size as u32,
                };

                self.blocks.insert(i + 1, new_block);
                return Some((&mut self.blocks[i + 1], empty_space));
            }
        }

        None
    }

    fn compact_block(&mut self, block: Block, allow_fragmentation: bool, max_index: usize) -> usize {
        let mut remaining_size: usize = block.size;

        while remaining_size > 0 {
            let min_block_size = if allow_fragmentation { 1 } else { block.size };
            if let Some((new_block, max_size)) = self.insert_empty_block(min_block_size, max_index) {
                new_block.file_id = block.file_id;
                if remaining_size <= max_size {
                    new_block.size = remaining_size;
                    remaining_size = 0;
                } else {
                    new_block.size = max_size;
                    remaining_size -= max_size;
                }
            } else {
                return remaining_size;
            }
        }

        0
    }

    fn compact(&mut self, allow_fragmentation: bool) {
        if allow_fragmentation {
            let mut compacted_last_block = true;
            while compacted_last_block {
                let block = self.blocks.last().unwrap().clone();
                let remaining_size = self.compact_block(block, allow_fragmentation, self.blocks.len());
                if remaining_size == 0 {
                    self.blocks.pop();
                    compacted_last_block = true;
                } else {
                    self.blocks.last_mut().unwrap().size = remaining_size;
                    compacted_last_block = false;
                }
            }
        } else {
            let mut index = self.blocks.len() - 1;
            while index > 0 {
                let block = self.blocks[index].clone();
                let remaining_size = self.compact_block(block, allow_fragmentation, index);
                if remaining_size == 0 {
                    self.blocks.remove(index + 1);
                } else {
                    index = index - 1;
                }
            }
        }
    }

    fn checksum(&self) -> u64 {
        self.blocks.iter().map(|b| b.checksum()).sum()
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut index = 0;
        for block in &self.blocks {
            while index < block.index {
                fmt.write_str(".")?;
                index += 1;
            }
            while index < block.index + block.size as u32 {
                write!(fmt, "{}", block.file_id)?;
                index += 1;
            }
        }

        Ok(())
    }
}

fn parse_input(input: &str) -> Disk {
    let trimmed = input.trim();
    let mut index: u32 = 0;
    let mut file_id = 0;
    let mut is_empty = false;
    let mut blocks: Vec<Block> = Vec::new();

    for char in trimmed.chars() {
        let size = char.to_digit(10).unwrap() as usize;
        if is_empty {
            index += size as u32;
        } else {
            let b = Block {
                file_id,
                size,
                index,
            };

            blocks.push(b);

            index += size as u32;
            file_id += 1;
        }

        is_empty = !is_empty;
    }

    Disk { blocks }
}

impl Solution for Day9 {
    fn solve_part1(input: &str) -> String {
        let mut disk = parse_input(input);
        disk.compact(true);
        
        disk.checksum().to_string()
    }

    fn solve_part2(input: &str) -> String {
        let mut disk = parse_input(input);
        disk.compact(false);
        
        disk.checksum().to_string()
    }
}
