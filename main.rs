//cmd sample application
use std::io;

fn main() {
    let a = 5;
    let b = a; //this is valid because this will copy 5
    let mut input = String::new();   //i put mut because we will be chainging the input 
    let mut s = input;
    io::stdin().read_line(&mut input);
    let mut  mar_weight: f32 = calculate_weight_on_mars(100.0);
     mar_weight = mar_weight *1000.0;
    println!("Weight on mars : {}kg",mar_weight);
    calculate_weight_on_mars(100.0);
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight/9.81) * 3.7
}
 
