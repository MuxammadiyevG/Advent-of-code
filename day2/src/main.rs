fn osish_bolsa(nums: Vec<i32>) -> bool {
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        if diff <= 0 || diff >3 {
            return false;
        }
    }
    true
}
fn kamayish_bolsa(nums: Vec<i32>) -> bool {
    for i in 0..nums.len() - 1 {
        let diff = nums[i] - nums[i + 1];
        if diff <= 0 || diff > 3 {
            return false;
        }
    }
true
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Faylni topa olmadi");
    let mut sana = 0;
    for (i, line) in input.lines().enumerate() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("intga o'tkazishda xato"))
            .collect();
        match nums[0].cmp(&nums[1]) {
            std::cmp::Ordering::Greater => {
                if nums.len() > 2 && kamayish_bolsa(nums.clone()) {
                    println!("Safe");
                    sana += 1;
                } else {
                    println!("Unsafe");
                }
            }
            std::cmp::Ordering::Less => {
                if nums.len() > 2 && osish_bolsa(nums.clone()) {
                    println!("Safe");
                    sana += 1;
                } else {
                    println!("Unsafe");
                }
            }
            std::cmp::Ordering::Equal => println!("Unsafe"),
        }
    }
    println!("jami soni {} ", sana);
}
