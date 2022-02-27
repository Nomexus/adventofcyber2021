use std::fs;

fn get_file_contents() -> String {
    let filename = "src/days/day1/input.txt";
    fs::read_to_string(filename).expect("Could not read file")
}

fn part1(lines: &Vec<&str>) {
    let (mut increases, mut previous) = (0, 0);

    for line in lines {
        let line_as_number = line.parse::<i32>().unwrap();

        if line_as_number > previous && previous > 0 {
            increases += 1;
        }

        previous = line_as_number;
    }

    println!("Number of increases Part 1: {}", increases);
}

fn part2(lines: &Vec<&str>) {
    let (mut increases, mut previous, mut line) = (0, 0, 0);

    'outer: loop {
        let mut sum = 0;

        for i in line..(line + 3) {
            if line == lines.len() {
                break 'outer;
            }

            sum += lines[i].parse::<i32>().unwrap();
            line += 1;
        }

        if previous > 0 && sum > previous {
            increases += 1;
        }

        previous = sum;
        line -= 2;
    }

    println!("Number of increases Part 2: {}", increases);
}

pub fn run() {
    let file_contents = get_file_contents();
    let lines: Vec<&str> = file_contents.split("\n").into_iter().collect();

    part1(&lines);
    part2(&lines);
}