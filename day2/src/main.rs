fn main() {
    let binding = String::from_utf8(include_bytes!("input.txt").to_vec()).unwrap();
    let input: Vec<_> = binding.split("\n").collect();

    for x in &input {
        let index = input.iter().position(|x| *x == " ");

        println!("{}", index);
    }

    println!("{:?}", input);
}
