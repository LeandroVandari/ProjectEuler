pub fn solve() -> String {
    let mut count = 1;
    let mut current_year = 1900;
    let mut month = 0;
    let mut dow = 0;
    while current_year != 2001 {
        let days_in_month = match month {
            0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
            3 | 5 | 8 | 10 => 30,
            1 => {
                if ((current_year % 4 == 0) && current_year % 100 != 0) || (current_year % 400 == 0)
                {
                    29
                } else {
                    30
                }
            }
            _ => unreachable!(),
        };
        dow += days_in_month;
        dow %= 7;
        if dow == 6 && current_year >= 1901{
            count += 1;
        }
        month += 1;
        if month == 11 {
            current_year += 1;
            month = 0;
        }

    }
    count.to_string()
}
