fn verify_password(from: &i32) -> bool {
    let from_vec: Vec<u32> = from.to_string().chars().map(|w| w.to_digit(10).unwrap()).collect();
//    let mut to_vec: Vec<i32> = to.to_string().split("").iter().map(|w| w.parse::<i32>()).collect();

    // 130254

    let mut adjacent_count: Vec<u32> = vec![0,0,0];
    let mut adjacent_group = 0;

    let mut has_adjacent = false;
    for index in 1..6 {
        if from_vec[index] < from_vec[index - 1] {
            has_adjacent = false;
            break;
        } else if from_vec[index] == from_vec[index - 1] {
            if has_adjacent && from_vec[index - 2] != from_vec[index] {
                adjacent_group += 1;
            }

            if adjacent_count[adjacent_group] == 0 {
                adjacent_count[adjacent_group] = 1;
            }

            adjacent_count[adjacent_group] += 1;
            has_adjacent = true;
        }
    }

    has_adjacent && adjacent_count.iter().filter(|&w| *w > 0 && *w == 2).count() > 0
}

pub fn main() {
    let from = 130254;
    let to = 678275;
    let mut count = 0;

    for i in from..to {
        if verify_password(&i) {
            count += 1;
            println!("{}", i);
        }
    }

    println!("{}", count);
}