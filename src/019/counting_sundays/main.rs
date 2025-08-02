
fn num_sundays_in_year() -> u32 {
    let mut starting_day = 1;
    // this will be the starting day of the year
    // Sunday is 0, Monday is 1, Tuesday is 2, Wednesday is 3
    // Thursday is 4, Friday is 5, Saturday is 6

    let mut sundays = 0;
    for year in 1901..=2000 {
        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        for month in 0..12 {
            if starting_day == 0 {
                sundays += 1;
            }

            match month {
                0|2|4|6|7|11 => starting_day = (starting_day + 3) % 7,
                3|5|8|9|10 => starting_day = (starting_day + 2) % 7,
                1 => if !leap {starting_day = (starting_day + 1) % 7},
                _ => panic!("Nonexistent month")
            }
        }

    }
    return sundays;
}

fn main() {
    let res = num_sundays_in_year();
    print!("{res}\n");
}
