// // 1. Define a trait named `Bite`
// //
// // Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// // want to bite something.  Once this trait is defined, you should be able to run the program with
// // `cargo run` without any errors.
// //
// //  trait Bite...

// trait Bite {
//     fn bite(self: &mut Self);
// }


// // 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// // need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// // use a different field, though).
// //
// // #[derive(Debug)] // include this line right before your struct definition
// // struct Grapes...

// #[derive(Debug)]
// enum Color {
//     Purple,
//     Green,
//     Black
// }

// #[derive(Debug)]
// struct Grapes {
//     quantity: i32,
//     color: Color,
// }


// impl Grapes {
//     fn new() -> Self {
//         Self { quantity: (100) }
//     }

//     fn length(&self) -> i32 {
//         self.quantity
//     }
// }


// // 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// // If you need a hint, look at how it was done for Carrot at the bottom of this file.
// //
// // impl Bite for...

// impl Bite for Grapes {
//     fn bite(self: &mut Self) {
//         self.quantity -= 1;
//     }
// }

// fn bunny_nibbles<T: Bite>(food: &mut T) {
//     for i in 0..10  {
//         food.bite();
//     }
// }

// fn main() {
//     // Once you finish #1 above, this part should work.
//     let mut carrot = Carrot { percent_left: 100.0 };
//     carrot.bite();
//     println!("I take a bite: {:?}", carrot);

//     // 4. Uncomment and adjust the code below to match how you defined your
//     // Grapes struct.
//     //
//     let mut grapes = Grapes { quantity: 100, color: Color::Green };
//     grapes.bite();
//     println!("Eat a grape: {:?}", grapes);

//     // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
//     // function that:
//     // - takes a mutable reference to any type that implements Bite
//     // - calls `.bite()` several times
//     // Hint: Define the generic type between the function name and open paren:
//     //       fn function_name<T: Bite>(...)
//     //
//     bunny_nibbles(&mut grapes);
//     println!("Bunny nibbles for awhile: {:?}", grapes);

// }

// #[derive(Debug)] // This enables using the debugging format string "{:?}"
// struct Carrot {
//     percent_left: f32,
// }

// impl Bite for Carrot {
//     fn bite(self: &mut Self) {
//         // Eat 20% of the remaining carrot. It may take awhile to eat it all...
//         self.percent_left *= 0.8;
//     }
// }









fn get_age_request()-> Option<i32> {
    Option::Some(26)
}


fn main () {
    let operation = Option::None::<usize>;
    let operation2 =  get_age_request();

    let test: Option<i32> = None;

    let mut test = None; // Datatype is not required to be specified if he can detect it under this line
    test = Some(5); // datatype is detected after declaration and it's working

    let _x = match operation  {
        Some(x) => {
            println!("I got a value : {}", x);
        },
        None => {
            println!("No value found"); // Triggered
        }
    };

    let _x = match operation2  {
        Some(x) => {
            println!("I got a value : {}", x); // Triggered
        },
        None => {
            println!("No value found");
        }
    };

    // Short version to get directly the value
    if let Option::Some(x) = operation2 {
        println!("I got a value : {}", x); // Triggered
    }
}