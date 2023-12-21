fn main() {
        let num = 25;

        println!("{} dividido por 5 = {}", num, divide_by_5(num));
}

fn divide_by_5(num: u32) -> u32 {
        num / 5
}