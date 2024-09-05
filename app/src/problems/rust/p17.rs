use std::collections::HashMap;

static NAMES_MAP: std::sync::LazyLock<HashMap<u32, &str>> = std::sync::LazyLock::new(|| {
    HashMap::from([
        (0, ""),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (100, "hundred"),
        (1000, "onethousand"),
    ])
});

pub fn solve() -> String {
    const MAX_NUM: u32 = 1000;

    (1..=MAX_NUM).fold(0, |acc, x| acc + get_number_name_length(x)).to_string()
}

fn get_number_name_length(number: u32) -> usize {
    if number <= 20 || number == 1000 {
        return NAMES_MAP.get(&number).unwrap().len();
    }
    let mut res = 0;
    if number >= 100 {
        res += NAMES_MAP.get(&(number / 100)).unwrap().len() + NAMES_MAP.get(&100).unwrap().len();
        if number % 100 != 0 {
            res += 3;
        }
    }

    let tens = number % 100;
    if tens <= 20 {
        res += NAMES_MAP.get(&tens).unwrap().len();
    } else {
        res += NAMES_MAP.get(&((tens / 10) * 10)).unwrap().len();
        res += NAMES_MAP.get(&(tens % 10)).unwrap().len();
    }

    return res;
}
