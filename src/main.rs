use std::{
    collections::{HashMap, HashSet},
    fs::{read, File},
    io::{self, BufRead},
    ops::Index,
};

use regex::Regex;

fn main() {
    day6();
}

fn day1() {
    let mut llst = Vec::new();
    let mut rlst = Vec::new();

    let prob = File::open("src/adventfiles/day1").unwrap();
    let reader = io::BufReader::new(prob);

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<&str> = line.split_whitespace().collect();

        llst.push(numbers[0].parse::<i32>().unwrap());
        rlst.push(numbers[1].parse::<i32>().unwrap());
    }

    llst.sort();
    rlst.sort();

    let mut diff = 0;
    let mut score = 0;
    for i in 0..llst.len() {
        diff += (rlst[i] - llst[i]).abs();

        score += llst[i] * (rlst.clone().into_iter().filter(|x| *x == llst[i]).count() as i32);
    }

    println!("{diff}");
    println!("{score}");
}

fn day2() {
    let mut count = 0;
    let prob = File::open("src/adventfiles/day2").unwrap();
    let reader = io::BufReader::new(prob);

    for line in reader.lines() {
        let line = line.unwrap();
        let strings: Vec<_> = line.split_whitespace().collect();
        let numbers: Vec<i32> = strings.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        // Maybe not optimal tc but who cares we're speedrunning this
        // remove loop to get p1 answer
        for i in 0..numbers.len() {
            let mut numbers2 = numbers.clone();
            numbers2.remove(i);

            let mut valid = true;
            let mut increasing = true;
            let mut decreasing = true;

            for i in 0..numbers2.len() - 1 {
                increasing = increasing && (numbers2[i] < numbers2[i + 1]);
            }

            for i in 0..numbers2.len() - 1 {
                decreasing = decreasing && (numbers2[i] > numbers2[i + 1]);
            }

            for i in 0..numbers2.len() - 1 {
                if (numbers2[i] - numbers2[i + 1]).abs() < 1
                    || (numbers2[i] - numbers2[i + 1]).abs() > 3
                {
                    valid = false;
                }
            }

            if valid && (increasing || decreasing) {
                count += 1;
                break;
            }
        }
    }

    println!("{count}");
}

fn day3() {
    let prob = File::open("src/adventfiles/day3").unwrap();
    let reader = io::BufReader::new(prob);
    let mut count = 0;
    let mulexpr = Regex::new(r"^mul\(([0-9]+)\,([0-9]+)\)").unwrap();
    let doit = Regex::new(r"^do\(\)").unwrap();
    let dont = Regex::new(r"^don\'t\(\)").unwrap();

    // Had to compress to 1 line for this to work
    for line in reader.lines() {
        let line = line.unwrap();
        /* Part 1
        for instance in mulexpr.find_iter(&line) {
            let cap = mulexpr.captures(&instance.as_str()).unwrap();
            let num1: i32 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let num2: i32 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            count += num1 * num2;
        }
        */
        let mut pos = 0;
        let mut can_do = true;
        while pos < line.len() {
            //println!("{}", &line[pos..]);
            if dont.is_match(&line[pos..]) {
                pos += 7;
                // println!("dont");
                can_do = false;
            } else if doit.is_match(&line[pos..]) {
                pos += 4;
                // println!("doit");
                can_do = true;
            } else if can_do && mulexpr.is_match(&line[pos..]) {
                let cap = mulexpr.captures(&line[pos..]).unwrap();
                let num1: i32 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let num2: i32 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                count += num1 * num2;
                println!("mulexpr {num1}, {num2}");
                pos += 6 + cap.get(1).unwrap().len() + cap.get(2).unwrap().len();
            } else {
                // println!("other");
                pos += 1;
            }
        }
    }

    println!("Part 1: {count}");
}

fn day4() {
    /*
    This is gonna be annoying.
    So diagonal forwards and backwards
    Like this
    .
     .
      .
       .
    or this
        .
       .
      .
     .

    Up/down, left/right, reversed as well
     */

    let prob = File::open("src/adventfiles/day4").unwrap();
    let reader = io::BufReader::new(prob);
    let mut count = 0;
    let mut count2 = 0;
    // Type conversion be like
    let arr: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let colmax = arr[0].len() as i32;
    let rowmax = arr.len() as i32;
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j] == 'X' {
                // 0 to 1
                if (j as i32) + 3 < colmax {
                    if arr[i][j + 1] == 'M' && arr[i][j + 2] == 'A' && arr[i][j + 3] == 'S' {
                        count += 1;
                    }
                }
                // 0 to -1
                if (j as i32) - 3 >= 0 {
                    if arr[i][j - 1] == 'M' && arr[i][j - 2] == 'A' && arr[i][j - 3] == 'S' {
                        count += 1;
                    }
                }
                // 1 to 0
                if (i as i32) + 3 < rowmax {
                    if arr[i + 1][j] == 'M' && arr[i + 2][j] == 'A' && arr[i + 3][j] == 'S' {
                        count += 1;
                    }
                }
                // -1 to 0
                if (i as i32) - 3 >= 0 {
                    if arr[i - 1][j] == 'M' && arr[i - 2][j] == 'A' && arr[i - 3][j] == 'S' {
                        count += 1;
                    }
                }
                // -1 to -1
                if (i as i32) - 3 >= 0 && (j as i32) - 3 >= 0 {
                    if arr[i - 1][j - 1] == 'M'
                        && arr[i - 2][j - 2] == 'A'
                        && arr[i - 3][j - 3] == 'S'
                    {
                        count += 1;
                    }
                }
                // -1 to 1
                if (i as i32) - 3 >= 0 && (j as i32) + 3 < colmax {
                    if arr[i - 1][j + 1] == 'M'
                        && arr[i - 2][j + 2] == 'A'
                        && arr[i - 3][j + 3] == 'S'
                    {
                        count += 1;
                    }
                }
                // 1 to -1
                if (i as i32) + 3 < rowmax && (j as i32) - 3 >= 0 {
                    if arr[i + 1][j - 1] == 'M'
                        && arr[i + 2][j - 2] == 'A'
                        && arr[i + 3][j - 3] == 'S'
                    {
                        count += 1;
                    }
                }
                // 1 to 1
                if (i as i32) + 3 < rowmax && (j as i32) + 3 < colmax {
                    if arr[i + 1][j + 1] == 'M'
                        && arr[i + 2][j + 2] == 'A'
                        && arr[i + 3][j + 3] == 'S'
                    {
                        count += 1;
                    }
                }
            } else if arr[i][j] == 'A' {
                if (i as i32) - 1 >= 0
                    && (i as i32) + 1 < rowmax
                    && (j as i32) - 1 >= 0
                    && (j as i32) + 1 < colmax
                {
                    if arr[i - 1][j - 1] == 'M' && arr[i + 1][j + 1] == 'S' {
                        if arr[i - 1][j + 1] == 'M' && arr[i + 1][j - 1] == 'S' {
                            count2 += 1;
                        } else if arr[i - 1][j + 1] == 'S' && arr[i + 1][j - 1] == 'M' {
                            count2 += 1;
                        }
                    } else if arr[i - 1][j - 1] == 'S' && arr[i + 1][j + 1] == 'M' {
                        if arr[i - 1][j + 1] == 'S' && arr[i + 1][j - 1] == 'M' {
                            count2 += 1;
                        } else if arr[i - 1][j + 1] == 'M' && arr[i + 1][j - 1] == 'S' {
                            count2 += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{count}, {count2}");
}

fn day5() {
    let prob = File::open("src/adventfiles/day5").unwrap();
    let reader = io::BufReader::new(prob);
    let splitter = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
    let mut pairs: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut count = 0;
    let mut count2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if splitter.is_match(&line) {
            let k = splitter
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let v = splitter
                .captures(&line)
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            pairs.entry(k).or_default().push(v);
        } else {
            if !line.is_empty() {
                let mut eval: Vec<_> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
                let mut valid = true;
                let mut is_changing = true;
                while is_changing {
                    is_changing = false;
                    for i in 0..eval.len() {
                        let num = eval[i];
                        if pairs.contains_key(&num) {
                            for v in pairs.get(&num).unwrap() {
                                if eval.contains(v) {
                                    if eval.iter().position(|x| x == v).unwrap() < i {
                                        let temp = eval[i];
                                        let j = eval.iter().position(|x| x == v).unwrap();
                                        eval[i] = eval[j];
                                        eval[j] = temp;
                                        is_changing = true;
                                        valid = false;
                                    }
                                }
                            }
                        }
                    }
                }
                if valid {
                    count += eval.get(eval.len() / 2).unwrap();
                } else {
                    count2 += eval.get(eval.len() / 2).unwrap();
                }
            }
        }
    }
    println!("{count}, {count2}");
}

fn day6() {
    let prob = File::open("src/adventfiles/day6").unwrap();
    let reader = io::BufReader::new(prob);
    let mut arr: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut keep_going = true; 
    let mut coor_map:HashSet<(i32, i32)> = HashSet::new();
    let rowmax = arr.len();
    let colmax = arr[0].len();
    
    let mut count = 0;

    while keep_going {
        for r in 0..arr.len() {
            for c in 0..arr[r].len() {
                if arr[r][c] == '^' {
                    if r == 0 {
                        keep_going = false;
                        if coor_map.insert((r as i32, c as i32)) {
                            count += 1;
                        }
                        break;
                    } else {
                        if arr[r-1][c] == '#' {
                            arr[r][c] = '>';
                        } else {
                            arr[r][c] = '.';
                            arr[r-1][c] = '^';
                            if coor_map.insert((r as i32, c as i32)) {
                                count += 1;
                            }
                        }
                    }
                } else if arr[r][c] == 'v' {
                    if r == rowmax - 1 {
                        keep_going = false;
                        if coor_map.insert((r as i32, c as i32)) {
                            count += 1;
                        }
                        break;
                    } else {
                        if arr[r+1][c] == '#' {
                            arr[r][c] = '<';
                        } else {
                            arr[r][c] = '.';
                            arr[r+1][c] = 'v';
                            if coor_map.insert((r as i32, c as i32)) {
                                count += 1;
                            }
                        }
                    }
                } else if arr[r][c] == '>' {
                    if c == colmax - 1 {
                        keep_going = false;
                        if coor_map.insert((r as i32, c as i32)) {
                            count += 1;
                        }
                        break;
                    } else {
                        if arr[r][c+1] == '#' {
                            arr[r][c] = 'v';
                        } else {
                            arr[r][c] = '.';
                            arr[r][c+1] = '>';
                            if coor_map.insert((r as i32, c as i32)) {
                                count += 1;
                            }
                        }
                    }
                } else if arr[r][c] == '<' {
                    if c == 0 {
                        keep_going = false;
                        if coor_map.insert((r as i32, c as i32)) {
                            count += 1;
                        }
                        break;
                    } else {
                        if arr[r][c-1] == '#' {
                            arr[r][c] = '^';
                        } else {
                            arr[r][c] = '.';
                            arr[r][c-1] = '<';
                            if coor_map.insert((r as i32, c as i32)) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{count}");
}
