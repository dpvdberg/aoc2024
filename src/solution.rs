pub trait Solution {
    fn solve_part1(&self, input: &str) -> String;
    fn solve_part2(&self, input: &str) -> String;
    fn solve(&self, input: &str) -> String {
        let part1 = self.solve_part1(&input);
        let part2 = self.solve_part2(&input);
        format!("part 1:\n{}\n\npart 2:\n{}", part1, part2)
    }
}
