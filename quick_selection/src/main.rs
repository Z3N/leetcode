use std::cmp::Ordering;

use rand::distributions::Uniform;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    //let mut vec = vec![1, 6, 2, 5, 1, 7, 4];
    for _ in 0..1000000000 {
        //let mut vec = vec![94, 32, 45, 32, 61, 49, 32, 97, 54, 83];
        let k = rand::thread_rng().gen_range(0..1000);
        let mut vec = rand::thread_rng()
            .sample_iter(Uniform::new(0, 1000))
            .take(k + 1)
            .collect::<Vec<_>>();
        let mut reference = vec.clone();
        //println!("{:?}, {}k=", etalon,k);
        reference.sort();
        let result = quick_selection(&mut vec, k);
        if reference[k] != result {
            println!("{k}");
            println!("{:?}, {}", reference, reference[k]);
            println!("{:?}, {}", vec, result);
            return;
        }
    }
}

fn quick_selection(vec: &mut [i32], select: usize) -> i32 {
    let vec_len = vec.len();

    let mut left = 0;
    let mut right = vec_len - 1;

    while right > left {
        let j = partition(&mut vec[left..=right]) + left;
        match j.cmp(&select) {
            Ordering::Greater => right = j - 1,
            Ordering::Equal => return vec[select],
            Ordering::Less => left = j + 1,
        }
    }
    vec[select]
}

fn partition(vec: &mut [i32]) -> usize {
    let tail = vec.len() - 1;
    let mut left = 0;
    let pivot = vec.len() / 2 - 1;
    let mut right = tail - 1;
    vec.swap(pivot, tail);
    loop {
        while vec[left] <= vec[tail] && left <= right {
            left += 1;
        }
        while vec[right] > vec[tail] && right > left {
            right -= 1;
        }
        if left >= right {
            break;
        }
        vec.swap(left, right);
        left += 1;
        right -= 1;
    }
    vec.swap(left, tail);
    left
}
