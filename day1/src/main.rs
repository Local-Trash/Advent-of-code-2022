fn main() {
    let binding = String::from_utf8(include_bytes!("input.txt").to_vec()).unwrap();

    let input: Vec<_> = binding.split("\n\n").collect();

    let mut mid: Vec<Vec<_>> = Vec::new();

    let mut output: Vec<i32> = Vec::new();

    for s in input {
        mid.push(s.split("\n").collect());
    }

    for x in mid {
        let mut z: i32 = 0;

        for y in x {
            z += y.parse::<i32>().unwrap();
        }

        output.push(z);
    }

    let counter

    print!("{:?}", output);
}
