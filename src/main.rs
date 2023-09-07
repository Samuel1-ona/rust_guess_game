use std::io;

use std::cmp::Ordering; // for compering and ordering (less, Greater, and equal)

use rand::Rng; // defines the methods that random number generator implement

fn main () {
   
    println! ("Guess thee number");
    let secret_number = rand::thread_rng() // The rand::thread_rng will give you the particular ramdom number generator

    .gen_range(1..101); // the random number generator that takes two arguments (1..101)

    loop {   // to loop the process
      
    
    println!("The secret number is: {}", secret_number );
    println! ("please input your guess.");

// storing values with variables ; Let statement is use to create a variable

   let mut guess = String :: new(); // new function create an empty string 
                                    // new function can be found  in  data type its commom to make new values

 

io::stdin().read_line(&mut guess)  // .read_line(&mut guess) it is a method to get input for the user
                                   // it takes whatever the user types in the stin and place it into a string ; it takes the strings as argument note: the string must me mutable
      .expect("failed to read line");

   let guess: u32 = match guess.trim().parse(){  // convert guess string to real number 
   
      Ok(num) => num,
      Err(_) => continue,
   };                                      
         
                                       
   println!("You guessed: {}", guess );

   // compare numerically to secret number to the guess, the guess shadow the previous guess therefore this feature is used to convert one type to another type
                              
   match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
         println!("You win");
         break; // to end after the correct guess i.e existing the loop 
      } 
    }
   }
}
