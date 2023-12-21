// Declare Car struct to describe vehicle with four named fields
struct Car {
        color: String,
        transmission: Transmission,
        convertible: bool,
        mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
        Manual,
        SemiAuto,
        Automatic,
}

// Build a "Car" by using values from the input arguments...
// - Color of a car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool, mileage: u32) -> Car {
    // Create a new "Car" instance with requested characteristics
    // - Corrected code: return a "Car" struct
    // - Bind first three fields to value of corresponding input argument
    // - Set mileage to 0
        Car {
                color: color,
                transmission: transmission,
                convertible: convertible,
                mileage: mileage
        }
}

fn main() {
    // Order three cars...
        let mut car = car_factory(String::from("Red"), Transmission::Manual, false, 0);
        println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

        car = car_factory(String::from("Blue"), Transmission::SemiAuto, true, 0);
        println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

        car = car_factory(String::from("Black"), Transmission::Automatic, true, 217);
        println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
}