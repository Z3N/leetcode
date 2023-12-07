use std::collections::HashSet;

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let mut bank = bank.into_iter().map(|gene| gene.into_bytes()).collect::<HashSet<_>>();

        if !bank.contains(end_gene.as_bytes()) {
            return -1;
        }
        let end_gene = end_gene.into_bytes();
        let codes = [b'A', b'C', b'G', b'T'];
        let mut current = vec![start_gene.into_bytes()];
        let mut next = Vec::new();
        let mut steps = 0;
        while !current.is_empty() {
            for mut gene in current.drain(..) {
                if gene == end_gene {
                    return steps;
                }
                for i in 0..gene.len() {
                    let skip_current_code = gene[i];
                    for code in codes.iter().filter(|&&x| x != skip_current_code) {
                        gene[i] = *code;
                        if let Some(gene) = bank.take(&gene) {
                            next.push(gene);
                        }
                        gene[i] = skip_current_code;
                    }
                }
            }
            std::mem::swap(&mut current, &mut next);
            steps += 1;
        }
        -1
    }
}

struct Solution;
