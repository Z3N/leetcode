use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
struct Denominator {
    id:    String,
    value: f64
}

impl Denominator {
    fn new(id: String, value: f64) -> Self {
        Self { id, value }
    }
}
impl PartialEq<Self> for Denominator {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Denominator {}
impl Hash for Denominator {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut map = std::collections::HashMap::new();
        for (mut equation, value) in equations.into_iter().zip(values) {
            let denominator = equation.swap_remove(1);
            let numerator = equation.swap_remove(0);
            map.entry(numerator.clone())
               .or_insert(HashSet::new())
               .insert(Denominator::new(denominator.clone(), value));
            map.entry(denominator)
               .or_insert(HashSet::new())
               .insert(Denominator::new(numerator, 1.0 / value));
        }
        queries.into_iter()
               .map(|query| Self::dfs(&map, query, &mut HashSet::new(), None).map_or(-1.0, |val| val))
               .collect()
    }

    fn dfs(graph: &HashMap<String, HashSet<Denominator>>, query: Vec<String>, visited: &mut HashSet<String>, result: Option<f64>) -> Option<f64> {
        visited.insert(query[0].clone());
        let entry = graph.get(&query[0])?;
        if query[0] == query[1] {
            return Some(1.0);
        }
        if let Some(direct_connect) = entry.iter().find(|&denom| denom.id == query[1]) {
            return result.map_or(Some(direct_connect.value), |value| Some(value * direct_connect.value));
        }
        for edge in entry {
            if visited.contains(&edge.id) {
                continue;
            }
            if let Some(path) = entry.iter().find(|&denom| denom.id == edge.id) {
                if let Some(found) = Solution::dfs(graph,
                                                   vec![edge.id.clone(), query[1].clone()],
                                                   visited,
                                                   result.map_or(Some(path.value), |value| Some(value * path.value)))
                {
                    return Some(found);
                }
            }
        }
        None
    }
}
struct Solution;
