use std::io;
// This is a code to convert celcius to fahrenheit and vise versa
// f = (c*1.8)+32
// c = (f-32)/1.8

fn main () {

    loop {

        println!("Enter '1' to convert Fahrenheit to Celsius");
        println!("Enter '2' to convert Celsius to  Fahrenheit");
        println!("Enter '3' to Exit");

        let mut choice = String::new();
    
        io::stdin().read_line(&mut choice).expect("please input either C or F");
        let choice :u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


     match choice {
        1 => {
            println!("Enter temprature in Fahrenheit:");
            let mut fahrenheit = String::new();

            io::stdin().read_line(&mut fahrenheit).expect("please enter a number");
            
     
            let fahrenheit :f32 = match fahrenheit.trim().parse() {
               Ok(num) => num,
               Err(_) => continue,

        };
        let celcius = (fahrenheit - 32.0)/ 1.8;
        println!("celcius is: {celcius}")

        },

        2 => {
            println!("Enter temprature in Celsius:");
            let mut celsius = String::new();

            io::stdin().read_line(&mut celsius).expect("please Enter a number");
            let celsius :f32 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,

            };

            let fahrenheit = (celsius * 1.8) + 32.0;
            println!("fahrenheit is: {fahrenheit}");
        },

        3 => {
            println!("Exiting the tem converter...");
            break;
        },

        _ => {
            println!("Invalid input!")
        }


    }
 }


}

