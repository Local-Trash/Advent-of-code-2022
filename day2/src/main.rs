fn main() {
    let binding = String::from_utf8(include_bytes!("input.txt").to_vec()).unwrap();
    let input: Vec<_> = binding.split("\n").collect();
    let mut output: Vec<Vec<char>> = Vec::new();
    let mut points: i32 = 0;

    for x in input {
        output.push(x.chars().collect());
    }

    for x in &mut output {
        x.remove(x.iter().position(|x| *x == ' ').expect("not found"));
    }

    for x in &output {
        match x[1] {
            'X' => points += 1,
            'Y' => points += 2,
            'Z' => points += 3,
            _ => panic!("Something went wrong, {}", x[1]),
        }

        match x[0] {
            'A' => match x[1] {
                'Y' => points += 6,
                'X' => points += 3,
                _ => println!("test"),
            },
            'B' => match x[1] {
                'Y' => points += 3,
                'Z' => points += 6,
                _ => println!("test"),
            },
            'C' => match x[1] {
                'Z' => points += 3,
                'X' => points += 6,
                _ => println!("test"),
            },
            _ => println!("test"),
        }

        println!("{}", points);
    }

    println!("{:?}", output);
    println!("{}", points);
}
