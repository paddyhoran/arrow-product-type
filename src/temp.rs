use std::ops::Add;

use indexmap::IndexMap;
/// Holds the actual values that are possible within a dimension.
#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct DimensionValues(Vec<String>);

/// The collection of all possible dimensions that a value **could** vary by.
#[derive(Default, Eq, PartialEq, Debug)]
pub struct PossibleDimensions(IndexMap<String, DimensionValues>);

impl PossibleDimensions {
    #[cfg(test)]
    pub fn add_dimension(mut self, name: String, values: Vec<String>) -> Self {
        self.0.insert(name, DimensionValues(values));
        self
    }
}

fn combine_dimensions(lhs: &PossibleDimensions, rhs: &PossibleDimensions) -> PossibleDimensions {
    let mut new_possible_dimensions = IndexMap::new();

    let mut lhs_iter = lhs.0.iter();
    let mut rhs_iter = rhs.0.iter();

    let mut l = lhs_iter.next();
    let mut r = rhs_iter.next();

    loop {
        if l.is_none() {
            l = lhs_iter.next();
        }
        if r.is_none() {
            r = rhs_iter.next();
        }

        match (l, r) {
            (None, None) => break,
            (None, Some((name, values))) => {
                let _ = new_possible_dimensions.insert(name.clone(), values.clone());
                for (k, v) in rhs_iter {
                    let _ = new_possible_dimensions.insert(k.clone(), v.clone());
                }
                break;
            }
            (Some((name, values)), None) => {
                let _ = new_possible_dimensions.insert(name.clone(), values.clone());
                for (k, v) in rhs_iter {
                    let _ = new_possible_dimensions.insert(k.clone(), v.clone());
                }
                break;
            }
            (Some((l_name, l_values)), Some((r_name, r_values))) => {
                if l_name == r_name {
                    if l_values == r_values {
                        new_possible_dimensions.insert(l_name.clone(), l_values.clone());
                        l = None;
                        r = None;
                    } else {
                        panic!("Dimension '{}' has conflicting values.", l_name)
                    }
                } else {
                    if l_values.0.len() == r_values.0.len() {
                        if l_name <= r_name {
                            new_possible_dimensions.insert(l_name.clone(), l_values.clone());
                            l = None;
                        } else {
                            new_possible_dimensions.insert(r_name.clone(), r_values.clone());
                            r = None;
                        }
                    } else if l_values.0.len() < r_values.0.len() {
                        new_possible_dimensions.insert(l_name.clone(), l_values.clone());
                        l = None;
                    } else {
                        new_possible_dimensions.insert(r_name.clone(), r_values.clone());
                        r = None;
                    }
                }
            }
        }
    }

    PossibleDimensions(new_possible_dimensions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_single_dimension() {
        let a = PossibleDimensions::default()
            .add_dimension("1".to_string(), vec!["a".to_string(), "b".to_string()]);
        let b = PossibleDimensions::default()
            .add_dimension("1".to_string(), vec!["a".to_string(), "b".to_string()]);

        let c = combine_dimensions(&a, &b);

        assert_eq!(c.0.len(), 1);
        assert_eq!(c, a);
    }
}
