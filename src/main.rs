struct Solution {}
impl Solution {
    fn value(&self) -> Simoleons {
        Simoleons(50)
    }
}
#[derive(Debug, PartialEq)]
struct Simoleons(usize);

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{Simoleons, Solution};

    #[test]
    fn metal_solution() {
        let solution: Solution = Solution {};
        let five_metal_value: Simoleons = Simoleons(50);
        assert_eq!(solution.value(), five_metal_value);
    }
}
