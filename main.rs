//adding libray for uinput and output
use std::io;
use colored::*;
//added use colored to add color to every line
//add a struct to hold out char info
struct Character{
    name: String,
    age: u8,
    health: u8,
    oxygen: u8,
    location: usize,
}
//struct to hold our location
//this will tell us about the loction we care in
struct Location{
    name:String,
     description: String,
     oxygen_level: u8,
     
}

// create fn that will allow char to move to new location
fn change_location1(){
    let msg: ColoredString = "Your move East landed you the treasure Map, Congrats!".bright_green();
    println!("{}",msg);

}

fn change_location2(){
   let msg: ColoredString = "\t\t  Alien envasion to the West kills you! Game Over".red().bold().italic();//by adding a space after the coma it will add the space before the anme
    println!("{}",msg);
    
    }

    fn change_location3(){
        let msg: ColoredString ="Massive Creater to the North, depleating most of you oxgyen, You must find reserves!".yellow();
        println!("{}",msg);
        }  

        fn change_location4(){
            let msg: ColoredString ="South took you off the map! Try again".bright_green();
            println!("{}",msg);
            }   
// create a fn to display a discription of loaction
fn describe_location(){
    let msg: ColoredString ="You are now on a beautiful cliff overlooking the planet Mars.".purple();
    println!("{}",msg);
    }  
    //also added fn for go up and go down
    fn go_up(character: &mut Character) {
        // Add logic for moving up to a higher location
        character.location += 1;
    
        let msg: ColoredString = "You Leveled up, Congrat.".cyan();
        println!("{}", msg);
    }
    
    fn go_down(character: &mut Character) {
        // Add logic for moving down to a lower location
        if character.location > 0 {
            character.location -= 1;
    
            let msg: ColoredString = "You descended to a lower level.".cyan();
            println!("{}", msg);
        } else {
            let msg: ColoredString = "You are already at the lowest level.".cyan();
            println!("{}", msg);
        }
    }
    
    
  
     

// create a fun of out char can take action
fn take_action(character: &mut Character){
    
// added print line for each direction 
    //tell the palyer to make a choice
    // changed the user input requirement to a single letter for direction
    let msg: ColoredString = "\n\n\t\t ----Type in what you would like to do,----\n\n".bright_green();
    println!("{}",msg);
    let msg: ColoredString = "\t\t L = Look around".bright_green();
    println!("{}",msg);
    let msg: ColoredString = "\t\t E = Go east".bright_green();
    println!("{}",msg);
    let msg: ColoredString = "\t\t W = Go west".bright_green();
    println!("{}",msg);
    let msg: ColoredString = "\t\t N = Go north".bright_green();
    println!("{}",msg);
    let msg: ColoredString = "\t\t S = Go south\n".bright_green();
    println!("{}",msg);
    let msg: ColoredString = "\t\t U = Go up".bright_green();
    println!("{}", msg);
    let msg: ColoredString = "\t\t D = Go down\n".bright_green();
    println!("{}", msg);

    // create a var to hold the users choice
    let mut action = String::new();
    //let mut game_over = false;
    // get users action choice
    io::stdin().read_line (&mut action).unwrap();
 
    //check users choice and take action
  // made users input lowercase to be sure matches what needs to be inputed 
    match action.trim().to_lowercase().as_str(){
    "l" => describe_location(),
    "e"=> change_location1(),
    "w" => change_location2(),
    "n"=> change_location3(),
    "s" => change_location4(),
    "u" => go_up(character),
     "d" => go_down(character),
    _=> println!("I dont understand that action command."),



         

};
    }

 



// create main fn which is out entry point into program
// fn will control the flow of our game 
// it will call other fn to do work
// it will also have our game loop

fn main(){
    //create a vector to hold our location
   // let mut locations =Vec::new();
   

    
    //create var to hold char
    let mut character = Character{
        name: String::from("Sam"),
        age: 45,
        health: 100,
        oxygen: 100,
        location: 0,
    };

    // Print out the title of our program
    let msg: ColoredString = "\n\n\t *** Welcome to the Mars sim! ***\n\n".bright_green();
    println!("{}",msg);
    // call our take action function
   //create a var to hold the users first name
// remeber mut is used to make the var mutable which means changeable

//I tabbed over to keep the writing on the page starting at the same spot
let mut first_name = String::new();
//ask for name
let msg: ColoredString ="\n\n\t\t Enter you name   \n\n".bright_green();
println!("{msg}");
//get the users input and store it in the first_name var
io::stdin().read_line(&mut first_name).expect("failed to read pt first name");
// trim or remove any extra spaces before and after the user input
//we do this becasue it make the compare easrier later on
first_name = first_name.trim().to_string();


//printing line to welcome to Mars
let msg: ColoredString = format!( "\t\t Welcome to Mars, {}", first_name).yellow().bold().italic();//by adding a space after the coma it will add the space before the anme
println!("{}",msg); 

let mut action = String::new(); // Move the action variable declaration outside the loop
// added a loop so all the fn could run and the program wouldnt stop after each answer
loop{
take_action(&mut character);
// Set our character's starting location
character.location = 0;  


}   

}





