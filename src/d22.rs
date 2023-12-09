use std::collections::HashMap;

pub fn exec(input: String) -> () {
    let mut count = 0;

    for line in input.lines() {
        let game = line.split(":").collect::<Vec<&str>>();
        let handfulls = game[1].split(";").collect::<Vec<&str>>();
        let mut min_colors: Vec<i32> = vec![0; 3];

        let mut colors = HashMap::new();

        colors.insert("red", 0);
        colors.insert("green", 1);
        colors.insert("blue", 2);

        for handfull in &handfulls {
            let bals = handfull.split(",").collect::<Vec<&str>>();
            for ball in &bals {
                for (key, value) in &colors {
                    if ball.contains(key) {
                        let info = ball.split(" ").collect::<Vec<&str>>();
                        if info[1].parse::<i32>().unwrap() > min_colors[value.clone()] {
                            min_colors[value.clone()] = info[1].parse::<i32>().unwrap();
                        }
                    }
                }
            }
        }

        count += min_colors.iter().product::<i32>();
    }
    println!("{}", count)
}
