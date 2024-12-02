pub trait Solution {
    fn solve_part1(input: &str) -> String;
    fn solve_part2(input: &str) -> String;
    fn solve(input: &str) -> String {
        let part1 = Self::solve_part1(&input);
        let part2 = Self::solve_part2(&input);
        format!("part 1:\n{}\n\npart 2:\n{}", part1, part2)
    }
}