advent_of_code::solution!(1);

pub fn dial1(mv: &str, start: i32) -> i32 {
    match mv.split_at(1) {
        ("L", num) => {
            let mut val = num.parse::<i32>().expect("Failed to parse number");
            val %= 100;
            let res = start - val;
            if res < 0 {
                return (100 + res) % 100;
            }
            res
        }
        ("R", num) => {
            let val = num.parse::<i32>().expect("Failed to parse number");
            (val + start) % 100
        }
        _ => panic!("invalid input"),
    }
}

pub fn dial2(mv: &str, start: i32) -> (i32, u64) {
    match mv.split_at(1) {
        ("L", num) => {
            let mut val = num.parse::<i32>().expect("Failed to parse number");
            let count = (val / 10).try_into().unwrap();
            val %= 100;
            let res = start - val;
            if res < 0 {
                ((100 + res) % 100, count)
            } else {
                (res, count)
            }
        }
        ("R", num) => {
            let val = num.parse::<i32>().expect("Failed to parse number");
            ((val + start) % 100, (val / 10).try_into().unwrap())
        }
        _ => panic!("invalid input"),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut starting: i32 = 50;
    let pwd = input.lines().fold(0, |acc, x| {
        starting = dial1(x, starting);
        match starting == 0 {
            true => acc + 1,
            false => acc,
        }
    });
    Some(pwd)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut starting: i32 = 50;
    let pwd = input.lines().fold(0, |acc, x| {
        let (new_starting, inter_acc) = dial2(x, starting);
        starting = new_starting;
        (match starting == 0 {
            true => acc + 1,
            false => acc,
        }) + inter_acc
    });
    Some(pwd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u64> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
