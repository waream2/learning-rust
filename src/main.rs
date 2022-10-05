fn main() {
    fn convert(measure: i32, unit_label: char) {
        if unit_label == 'c' {
            println!("{} F", measure as f32 * 1.8 + 32.0)
        } else if unit_label == 'f' {
            println!("{} C", (measure as f32 - 32.0) * 555.6)
        } else {
            println!("please enter either c or f")
        }
    }

    convert(32, 'f');

    fn days_of_xmas() {
        let gifts = [
            "Tree", "Leaves", "Ducks", "Chickens", "Pigs", "Macbooks", "Dells", "TVs", "Frames",
            "Legos", "Doors", "Dressers",
        ];

        for (i, gift) in gifts.iter().enumerate() {
            if i + 1 == 1 {
                println!(
                    "On The {}st day of Christmas my true love gave to me, {} {}. \n",
                    i + 1,
                    i + 1,
                    gift
                )
            } else if i + 1 == 2 {
                println!(
                    "On The {}nd day of Christmas my true love gave to me, {} {}. \n",
                    i + 1,
                    i + 1,
                    gift
                )
            } else if i + 1 == 3 {
                println!(
                    "On The {}rd day of Christmas my true love gave to me, {} {}. \n",
                    i + 1,
                    i + 1,
                    gift
                )
            } else {
                println!(
                    "On The {}th day of Christmas my true love gave to me, {} {}. \n",
                    i + 1,
                    i + 1,
                    gift
                )
            }
        }
    }

    days_of_xmas();

    fn generate_fibonacci(n: usize) {
        let fibo = [
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811,
        ];

        let answer = fibo[n];
        println!("{}", answer)
    }

    generate_fibonacci(10);
}
