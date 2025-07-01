use std::fs;
fn main() {
    let data = fs::read_to_string("input.txt").expect("Fayl topilmadi");
    let lines = data.trim().split("\n").collect::<Vec<_>>();
    let mut a: Vec<_> = Vec::new();
    let mut b: Vec<_> = Vec::new();
    for i in lines {
        let into_line = i.trim().split("   ").collect::<Vec<_>>();
        a.push(into_line[0].trim().parse::<i32>().unwrap());
        b.push(into_line[1].trim().parse::<i32>().unwrap());
    }
    a.sort();
    b.sort();
    let mut sum_ = 0;

    for i in 0..a.len() {
        if a[i] > b[i] {
            sum_ += a[i] - b[i];
        } else if a[i] < b[i] {
            sum_ += b[i] - a[i];
        } else {
            sum_ += 0;
        }
    }
    println!("sum_ = {}", sum_);
}
