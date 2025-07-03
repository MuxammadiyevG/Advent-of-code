use regex::Regex;
fn read_file(file_path: &str) -> String {
    let content = std::fs::read_to_string(file_path);
    match content {
        Ok(data) => data,
        Err(e) => String::from(format!("faylni o'qishda xatolik: {}", e)),
    }
}

fn extract_mul(mul: &str) -> i32 {
    let my_pattern2 = r"(?<first>[0-9]*)\,(?<second>[0-9]*)";
    let re = Regex::new(my_pattern2).expect("Bu yerda ham nimadur xato");
    let Some(caps) = re.captures(mul) else {
        println!("no match!");
        return 0;
    };
    &caps["first"].parse::<i32>().unwrap() * &caps["second"].parse::<i32>().unwrap()
}

fn main() {
    let my_pattern = r"mul\([0-9]*\,[0-9]*\)";
    let re = Regex::new(my_pattern).expect("Nimadur xato");
    let file_content = read_file("input.txt");
    //println!("{:?}", file_content);
    let caps: Vec<&str> = re.find_iter(&file_content).map(|f| f.as_str()).collect();
    let mut sum = 0;
    for i in &caps {
        sum += extract_mul(i);
    }
    println!("{:?}", caps);
    println!("{:?}", extract_mul(caps[0]));
    println!("Sum = {:?}", sum);
}
