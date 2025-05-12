fn main() {
    // Class 126
    let registration = (true, false, true);
    let first = registration.0;
    println!("First: {} and {:?} ", first, registration);

    let languages = (String::from("Rust"), String::from("Python"));
    let first = &languages.0;
    println!("First: {} and {:?} ", first, languages);

}
/*
 //Class 125
    let city = create_city();
    println!("{}", city);

    
//Class 125 outside main
fn create_city() -> String {
    String::from("New York")
}
*/



/*
  //Class 124
    let mut coffee = String::from("mocha");    
    let a = &mut coffee;
    println!("{}", a);
    let b = a;
    println!("{}", b);
*/
/*
    // Class 122
    let car = String::from("red");
    let ref1 = &car;
    let ref2 = &car;
    println!("{} and {} and {}.", ref1, ref2, &car);
*/
/*
//class 121
let mut current_meal = String::new();

add_flour(&mut current_meal);
show_my_meal(&current_meal);
*/
//Class 121 functions outside main
//meal: String
//mut meal : String
//meal: &String
//meal: &mut String
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {}", meal);
}