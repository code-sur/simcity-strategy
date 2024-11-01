fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Solution {}

struct Fabric {}

fn solve(available_factories: Vec<Fabric>) -> Solution {
    Solution {}
}

#[cfg(test)]
mod tests {
    use crate::{solve, Fabric, Solution};

    #[test]
    fn test() {
        let available_factories: Vec<Fabric> = Vec::new();
        let minimal_solution: Solution = Solution {};
        assert_eq!(solve(available_factories), minimal_solution);
    }
}
