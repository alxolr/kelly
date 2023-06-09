#[derive(Debug, PartialEq, PartialOrd)]
pub struct KellyAssumption(pub f32, pub f32);

#[derive(Debug, PartialEq)]
pub struct KellyFormulaBuilder {
    pub assumptions: Vec<KellyAssumption>,
}

impl Default for KellyFormulaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl KellyFormulaBuilder {
    pub fn new() -> Self {
        KellyFormulaBuilder {
            assumptions: vec![],
        }
    }

    pub fn set_assumptions(mut self, assumptions: Vec<KellyAssumption>) -> KellyFormulaBuilder {
        self.assumptions = assumptions;

        self
    }

    pub fn calculate(self) -> f32 {
        let max_wagger = self
            .assumptions
            .iter()
            .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
            .unwrap_or(self.assumptions.first().unwrap());

        let edge = self.assumptions.iter().fold(0.0, |mut acc, it| {
            acc += it.0 * it.1;

            acc
        });

        edge / max_wagger.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_kelly_assumptions() {
        let expected = KellyFormulaBuilder {
            assumptions: vec![
                KellyAssumption(0.8, 21.0),
                KellyAssumption(0.1, 7.5),
                KellyAssumption(0.1, -1.0),
            ],
        };

        assert_eq!(
            KellyFormulaBuilder::new().set_assumptions(vec![
                KellyAssumption(0.8, 21.0),
                KellyAssumption(0.1, 7.5),
                KellyAssumption(0.1, -1.0),
            ]),
            expected
        );
    }

    #[test]
    fn test_should_return_the_number_for_the_allocation() {
        let assumptions = vec![
            KellyAssumption(0.8, 21.0),
            KellyAssumption(0.1, 7.5),
            KellyAssumption(0.1, -1.0),
        ];
        let kelly = KellyFormulaBuilder::new().set_assumptions(assumptions);

        assert_eq!(kelly.calculate(), 0.8309524);
    }
}
