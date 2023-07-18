pub fn sort<T>(vec: &mut [T]) where T: Ord {
    let pivot = vec.len() / 2;
    let (less, greater) = partition(vec, pivot);
    if less.len() > 1 {
        sort(less);
    }
    if greater.len() > 1 {
        sort(greater)
    }
}

fn partition<T>(vec: &mut [T], pivot: usize) -> (&mut [T], &mut [T])
                where T: Ord
{
    vec.swap(0, pivot);
    let mut equal = 1;
    let mut less = 0;
    let mut great = vec.len();
    // less than pivot vec[0..less]
    // equal to pivot vec[less..=great]
    // great than pivot vec[great+1..]
    while equal < great {
        match vec[equal].cmp(&vec[less]) {
            std::cmp::Ordering::Less => {
                vec.swap(equal, less);
                less += 1;
                equal += 1;
            }
            std::cmp::Ordering::Equal => {
                equal += 1;
            }
            std::cmp::Ordering::Greater => {
                vec.swap(great - 1, equal);
                great -= 1;
            }
        }
    }
    let (leading, greater_than_pivot) = vec.split_at_mut(great);
    let (less_than_pivot, _) = leading.split_at_mut(less);
    (less_than_pivot, greater_than_pivot)
}

#[cfg(test)]
mod tests {
    use rand::distributions::Uniform;
    use rand::Rng;

    use super::*;

    #[test]
    fn it_works() {
        //let mut str = String::from("pbaxwppvpdycyz").chars().collect::<Vec<char>>();
        //partition::<char>(&mut str,0);
        //sort(&mut str);
        for _ in 0..1000000000 {
            //let mut vec = vec![94, 32, 45, 32, 61, 49, 32, 97, 54, 83];
            let k = rand::thread_rng().gen_range(0..1000);
            let mut vec = rand::thread_rng()
                .sample_iter(Uniform::new(0, 1000))
                .take(k + 1)
                .collect::<Vec<_>>();
            let mut reference = vec.clone();
            reference.sort();
            sort(&mut vec);
            if reference != vec {
                println!("{:?}, {:?}", reference, vec);
                assert!(true);
                panic!()
            }
        }
    }
}
