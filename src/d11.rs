pub fn exec(input: String) -> () {
    let mut count = 0;

    for line in input.lines() {
        let mut number = 0;
        for chara in line.chars() {
            if chara.is_numeric() {
                number = number + chara.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for chara in line.chars().rev() {
            if chara.is_numeric() {
                number = number + chara.to_digit(10).unwrap();
                break;
            }
        }
        count += number;
    }

    println!("{}", count);
}
