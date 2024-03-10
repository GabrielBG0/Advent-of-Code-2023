use std::collections::HashMap;

pub fn exec(input: String) -> () {
    let mut count = 0;

    let mut colors = HashMap::new();

    colors.insert("red", 12);
    colors.insert("green", 13);
    colors.insert("blue", 14);

    for line in input.lines() {
        let game = line.split(":").collect::<Vec<&str>>();
        let handfulls = game[1].split(";").collect::<Vec<&str>>();
        let mut found = false;
        for (key, value) in &colors {
            for handfull in &handfulls {
                let bals = handfull.split(",").collect::<Vec<&str>>();
                for ball in &bals {
                    if ball.contains(key) {
                        let info = ball.split(" ").collect::<Vec<&str>>();
                        if info[1].parse::<i32>().unwrap() > value.clone() {
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            println!("{}", game[0]);
            let game_number = game[0].split(" ").collect::<Vec<&str>>()[1];
            count += game_number.parse::<i32>().unwrap();
        }
    }
    println!("{}", count)
}
