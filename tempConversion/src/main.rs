use std::io;

//general functions 
fn input_to_str() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input);
    return input.trim().to_string();
}

fn main() {
    println!("Welcome to the weather station! We would love to help with your temperature conversion");
    
    println!("Is the data in C or F?");
    let uom = get_temp_uom();

    println!("What is the temp?");
    let temp: f32 = get_temp();

    temp_conversion_controller(&uom, temp)
    
}


// Temp Unit of Measure(uom) and Temp Inputs
fn get_temp_uom () -> String {
    let uom:String = input_to_str().to_uppercase();

    if validate_temp_input(&uom) {
        return uom
    } else {
        println!("Please enter a valid unit of measure C or F");
        //recursive until valid selection is found.
        get_temp_uom()
    }
}

fn get_temp () -> f32 {
    let temp: String = input_to_str();
    match temp.parse::<f32>(){
        Ok(i) => i,
        Err(_) => {
            println!("Invalid Temperature, please enter a valid temperature.");
            //loop back until valid temp is entered
            get_temp()
        }
    }
}


// Input Validation
fn validate_temp_input (uom: &str) -> bool {
    match uom {
        "F" | "C"=> {return true},
        _ => {return false},
    }
}


//Conversion 
fn convert_c_to_f(temp_c: f32) -> f32{
    let temp_f = (temp_c * 1.8) + 32.0;
    temp_f
}

fn convert_f_to_c(temp_f: f32) -> f32{
    let temp_c = (temp_f -32.0) * 0.5556;
    temp_c
}

fn temp_conversion_controller(unit: &str, temp: f32) {
    let converted_temp: f32;

    match unit {
        "F" => {
            converted_temp = convert_f_to_c(temp);
            println!("{}F is {}C",temp, converted_temp)
        }
        "C" => {
            converted_temp = convert_c_to_f(temp);
            println!("{}C is {}F", temp, converted_temp)
        }
        _ => {
            println!("Error")
        }
    }
}