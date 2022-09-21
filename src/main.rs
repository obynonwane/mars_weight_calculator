use std::io;
fn main() {
    let mut input = String::new();

    //passing mutable reference
    some_fn(&mut input);

    //two imutable references
    let input1 = &input;
    let input2 = &input;

    println!("{}, {}", input1, input2);

    io::stdin().read_line(&mut input);
    let mut mars_weight = calculate_weight_on_mars(100.00);
     mars_weight = mars_weight * 1000.0;
    println!("Weight on mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight /9.81)* 3.711
}

fn some_fn(_s: &mut String) {

}