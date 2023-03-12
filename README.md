# Kelly Formula

This is a simple implementation of the Kelly Formula in Rust. The Kelly Formula is a method of determining the optimal fraction of your bankroll to bet on a given bet. It is based on the assumption that you have an edge over the bookmaker, and that you can accurately estimate the probability of winning a given bet.

## Usage

```rust
use kelly::{ KellyAssumption, KellyFormulaBuilder };

fn main() {
    let assumptions = vec![
        // Create a KellyAssumption with a probability of 0.8 and odds of 21.0
        KellyAssumption(0.8, 21.0),
        // Create a KellyAssumption with a probability of 0.1 and odds of 7.5
        KellyAssumption(0.1, 7.5),
        // Create a KellyAssumption with a probability of 0.1 and odds of -1.0
        KellyAssumption(0.1, -1.0),
    ];

    // Create a new KellyFormulaBuilder with the assumptions
    let kelly = KellyFormulaBuilder::new().set_assumptions(assumptions);

    // Calculate the optimal fraction of your bankroll to bet
    assert_eq!(kelly.calculate(), 0.8309524);
}

```
