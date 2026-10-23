use std::collections::HashMap;
use std::time::Instant;

pub fn solve_two_pile_partition(s: &[i32]) -> (String, Vec<i32>, Vec<i32>) {
    let mut pile_a = Vec::new();
    let mut pile_b = Vec::new();
    let mut pile_c = Vec::new();

    let mut chunks = s.chunks_exact(3);
    let mut idx = 0;
    
    for chunk in &mut chunks {
        let mut triplet = [chunk[0], chunk[1], chunk[2]];
        triplet.sort_unstable();
        let [small, medium, large] = triplet;

        if idx % 2 == 1 {
            pile_a.push(large);
            pile_b.push(small);
        } else {
            pile_a.push(small);
            pile_b.push(large);
        }
        pile_c.push(medium);
        idx += 1;
    }

    let rem = chunks.remainder();
    if !rem.is_empty() {
        let mut triplet = [0; 3];
        for (i, &val) in rem.iter().enumerate() {
            triplet[i] = val;
        }
        triplet.sort_unstable();
        let [small, medium, large] = triplet;
        if idx % 2 == 1 { pile_a.push(large); pile_b.push(small); } 
        else { pile_a.push(small); pile_b.push(large); }
        pile_c.push(medium);
    }

    let sum_a: i32 = pile_a.iter().sum();
    let sum_b: i32 = pile_b.iter().sum();
    let sum_c: i32 = pile_c.iter().sum();

    let target_doubled = sum_b + sum_c - sum_a;
    if target_doubled % 2 != 0 {
        return ("no".to_string(), vec![], vec![]);
    }

    let target_sum_c_to_a = target_doubled / 2;
    if target_sum_c_to_a < 0 || target_sum_c_to_a > sum_c {
        return ("no".to_string(), vec![], vec![]);
    }

    let mut dp = HashMap::new();
    dp.insert(0, None);

    for &item in &pile_c {
        let current_keys: Vec<i32> = dp.keys().cloned().collect();
        for current_sum in current_keys {
            let new_sum = current_sum + item;
            if new_sum <= target_sum_c_to_a && !dp.contains_key(&new_sum) {
                dp.insert(new_sum, Some(item));
            }
        }
        if dp.contains_key(&target_sum_c_to_a) {
            break;
        }
    }

    if dp.contains_key(&target_sum_c_to_a) {
        let mut c_to_a = Vec::new();
        let mut curr = target_sum_c_to_a;
        
        while curr > 0 {
            if let Some(&Some(item)) = dp.get(&curr) {
                c_to_a.push(item);
                curr -= item;
            }
        }

        let mut pile_c_pool = pile_c.clone();
        for item in &c_to_a {
            if let Some(pos) = pile_c_pool.iter().position(|x| x == item) {
                pile_c_pool.swap_remove(pos);
            }
        }

        pile_a.extend(c_to_a);
        pile_b.extend(pile_c_pool);

        return ("yes".to_string(), pile_a, pile_b);
    }

    ("no".to_string(), vec![], vec![])
}

fn main() {
    let sample_data = vec![
        49, 44, 38, 15, 34, 29, 21, 15, 31, 35, 25, 30, 21, 11, 36, 49, 61, 75, 52, 46, 39, 32, 21,
        27, 62, 54, 42, 29, 42, 14, 59, 39, 45, 28, 42, 15, 12, 21, 34, 14, 38, 29, 59, 50, 40, 48,
        40, 25, 52, 67, 39, 30, 11, 24, 10, 20, 27, 55, 40, 66, 31, 20, 25, 47, 59, 66, 27, 35, 14,
        34, 19, 24, 37, 17, 30, 29, 46, 41, 60, 36, 45, 19, 28, 12, 28, 19, 35, 44, 58, 39, 34, 44,
        56, 45, 57, 31, 45, 40, 108,
    ];

    let start = Instant::now();
    let (status, p1, p2) = solve_two_pile_partition(&sample_data);
    let duration = start.elapsed();

    println!("Status: {}", status);
    println!("Pile 1 Sum: {} | Pile 2 Sum: {}", p1.iter().sum::<i32>(), p2.iter().sum::<i32>());
    println!("Execution Duration: {:?}", duration);
}

