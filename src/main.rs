use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let score = parse_input!(input_line, i32);

    let three_points_max = score / 3;

    for i in (0..=three_points_max).rev() {
        let remaining_score_3 = score - i * 3;
        let five_points_max = remaining_score_3 / 5;
        for j in (0..=five_points_max).rev() {
            let remaining_score_5 = remaining_score_3 - j * 5;
            let seven_points_max = remaining_score_5 / 7;
            for k in (0..=seven_points_max).rev() {
                let remaining_score_7 = remaining_score_5 - k * 7;
                if remaining_score_7 == 0 {
                    println!("try: {}, converted: {}, drops/penalties: {}", k + j, k, i);
                }
            }
        }
    }
}
