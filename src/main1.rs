// fn main() {
//     println!("Hello, world!");
// }

fn main() {
    let formal = "Goodbye...!";
    let casual = "See you later...!!!";

    goodbye(formal);
    goodbye(casual);
}

fn goodbye(message: &str) {
    println!("\n{}", message);
}
