use std::io;
fn main() {
    let mut input = String::new();

    
    //passing mutable reference
    some_fn(&mut input);

    //two imutable references
    // let input1 = &input;
    // let input2 = &input;

    // println!("{}, {}", input1, input2);

    io::stdin().read_line(&mut input);
    println!("Input: {}", input);
    let mut mars_weight = calculate_weight_on_mars(100.00);
     mars_weight = mars_weight * 1000.0;
    println!("Weight on mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight /9.81)* 3.711
}

fn some_fn(s: &mut String) {

}
//mutably borrow or immutably borrow 
//add ampersand (&) to the variable type to make it point to a reference without taking ownership e.g s: &String
//in rust passing references as parameters is called borrowing
//passing a reference we have to add ampersand to the variable when passing it
//references are immutable by default
//we have two types of references a) Mutable refrence (&mut variableName) 
//and immutable refrence b) (&VariableName)
//you can only have a single mutable borrow 
//and you can also have many imutable borrow
//but you can not have both at same time