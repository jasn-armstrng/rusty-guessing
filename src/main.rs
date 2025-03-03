// -----------------------------------------------------------------------------
// Name: rusty-guessing
// Date: 2025-03-03
// Source: The Rust Programming Language, Chapter 2: Programming a Guessing Game
// Description: The program will generate a random integer between 1 and 100. It
//              will prompt the player to enter a guess. After teh guess is entered
//              the program will indicate if the guess is too low, or too high.
//              If the guess is correct. The game will print a congratulatory
//              message and exit.
// -----------------------------------------------------------------------------
use rand::Rng;
use std::{
    cmp::Ordering,
    io,
};

// main()
// -----------------------------------------------------------------------------
// When you start a Rust program, it begins execution in a single thread called
// the main thread. This thread starts with the main() function that serves as
// the entry point to your program.
fn main() {
    // Message to user
    println!("Guess the number!");

    // rand::rng() creates a handle/reference to a `thread-local` random number
    // generator.
    // ...
    // "Thread-local" means that each of these workers (threads) gets their own
    // personal copy of something that only they can access. Other workers can't
    // see or modify it.
    // ...
    // For the random number generator in Rust:
    // * Each thread gets its own private random number generator
    // * When Thread A generates random numbers, it uses its own generator
    // * When Thread B generates random numbers, it uses a completely different
    //   generator
    // * These generators don't interfere with each other
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // thread_rng
    // -------------------------------------------------------------------------
    // ThreadRng is an object (specifically, it's a struct in Rust). It's not a
    // process or facility, but rather a concrete type that:
    // * Contains the state needed to generate random numbers
    // * Implements the Rng trait (which defines methods for generating random values)
    // * Manages its own internal state as you request random numbers from it
    // * When you call rand::thread_rng(), you receive a handle (basically a reference)
    //   to this object that lives in your current thread's local storage. The object
    //   stays alive and maintains its state between calls, which allows it to produce
    //   a continuous stream of random numbers without repeating patterns.

    // Loop indefinitely.
    // loop is used to define the simplest kind of loop supported in Rust. It
    // runs the code inside it until the code uses break or the program exits.
    loop { // Event loop
        // Prompt user for guess
        println!("Please input your guess.");

        // Create a new empty string. This will not allocate any initial buffer.
        // Please see documentation for caveats.
        let mut guess = String::new();
        // Is there "garbage collection" for the input data that goes out of scope
        // after each iteration of the loop?
        // ---------------------------------------------------------------------
        // In Rust, there's no garbage collection like you'd find in languages such
        // as Java, Python, or JavaScript. Instead, Rust uses a system called "ownership"
        // with deterministic memory management.
        // ...
        // For your input data scenario:
        // * When you create let mut guess = String::new() inside the loop, memory
        //   is allocated on the heap for that string.
        // * When the loop iteration ends, that guess variable goes out of scope.
        // * At this precise moment, Rust automatically calls the drop function
        //   (similar to a destructor) for the String, which frees the memory that
        //   was allocated for it.
        // This is part of Rust's RAII (Resource Acquisition Is Initialization)
        // pattern, where resources are tied to an object's lifetime. When that
        // object goes out of scope, its destructor is called automatically, and
        // its resources are released.

        // io::stdin()
        // ---------------------------------------------------------------------
        // Constructs a new handle to the standard input of the current process.
        // Each handle returned is a reference to a shared global buffer whose
        // access is synchronized via a mutex. If you need more explicit control
        // over locking, see the [Stdin::lock] method.

        // "Current process" above refers to the process in which your Rust pro-
        // gram is currently executing. Every process has standard input (stdin),
        // standard output (stdout), and standard error (stderr) streams connected
        // to it by the operating system.

        // The "shared global buffer" is a memory area that:
        // * Is accessible from anywhere in your program
        // * Temporarily holds data that's being read from standard input
        // * Exists as a single instance for the entire program
        // * Can be accessed by multiple parts of your code
        // When you call io::stdin() multiple times in your program, you're not
        // creating multiple independent input streams. Instead, you're getting
        // multiple handles (references) to the same underlying input buffer.
        // ...
        // The shared global buffer is Rust's way of providing a convenient, safe,
        // and thread-safe interface to the underlying OS stream. It acts as a
        // middleman that handles the raw input from the OS and presents it to your
        // code in a more manageable way, while ensuring thread safety through the
        // mutex mechanism.

        // A mutex (short for "mutual exclusion") is a synchronization mechanism that:
        // * Ensures only one part of your program can access the buffer at a time
        // * Prevents data corruption that could happen if multiple threads tried to
        //   read from stdin simultaneously
        // * Works like a lock - when one part of your code is using stdin, other parts
        //   must wait until it's done
        // ...
        // This synchronization is necessary because:
        // * Multiple threads in your program might try to read from stdin at the
        //   same time
        // * Without synchronization, they could interfere with each other, causing
        //   errors or unexpected behavior
        // * The mutex ensures that each thread gets complete, consistent data when
        //   it reads from stdin
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!"); // Returns the contained [Ok] value,
                                             // consuming the self value.

        // Is data from stdin by default a string type?
        // ---------------------------------------------------------------------
        // Data from stdin is indeed received as string-like data by default in
        // Rust.
        // When you read from standard input using functions like io::stdin().read_line(&mut guess),
        // what you're getting is text data that's read as bytes and interpreted
        // as a string. The data is stored in a String type, which is Rust's UTF-8
        // encoded string type.
        // The conversion from string to number isn't automatic - you must explicitly
        // parse it, which is why the code uses guess.trim().parse(). The parse()
        // method (below) attempts to convert the string into whatever type you've
        // specified (in this case, u32).

        // parse():
        // ---------------------------------------------------------------------
        // - Parses this string slice into another type.
        // - parse can parse into any type that implements the FromStr trait.
        // - Here u32 implements the FromStr trait. It fails with a
        //   `type Err = ParseIntError`
        // - parse returns a Result type and Result is enum that has the variants
        //   Ok and Err.
        // - parse will return [Err] if it's not possible to parse this string
        //   slice into the desired type.
        let guess: u32 = match guess.trim().parse() {
            // `num` is just a temporary variable that holds the successfully
            // parsed number during the pattern matching process. It's a way to
            // "unwrap" the value from inside the Ok variant of the Result.
            Ok(num) => num,
            // Catch all (`_`) parsing errors and continue. We ignore invalid
            // input so the user can keep guessing. This is acceptable behavior
            // for this type of user interaction; in other interactions you may
            // want the program to crash or let the user know that that something
            // bad was entered.
            Err(_) => continue,
        };

        // Variable shadowing of `guess`:
        // When you first declare let mut guess = String::new();, you create
        // a mutable variable named guess that holds a String value.
        // Then when you write let guess: u32 = match guess.trim().parse() { ... },
        // you're creating a new variable that:
        // * Has the same name (guess)
        // * Has a different type (u32 instead of String)
        // * Shadows (or hides) the previous guess variable
        //
        // This is an idiomatic use of shadowing in Rust. It lets you reuse the
        // same variable name while converting a value from one type to another,
        // which is cleaner than creating differently named variables like
        // guess_string and guess_number.
        // [Important]
        // After this shadowing, any references to guess in the subsequent code
        // will refer to the new u32 variable, not the original String. The original
        // String variable is still in memory until it goes out of scope, but you
        // can no longer access it by the name guess.

        println!("You've guessed: {guess}");  // guess which previously was a
                                              // string accepted from stdin is
                                              // now of type u32

        // Ordering, cmp
        // ---------------------------------------------------------------------
        // In Rust, an Ordering is an enum from the standard library that repre-
        // sents the three possible outcomes when comparing two values: less than,
        // equal to, or greater than. It's defined in the std::cmp module.
        // The Ordering enum has three variants:
        //
        // * Ordering::Less - indicates that the first value is less than the
        //   second
        // * Ordering::Equal - indicates that the values are equal
        // * Ordering::Greater - indicates that the first value is greater than
        //   the second
        //
        // When a type implements the Ord trait (which requires implementing the
        // ------------------------------------
        // cmp method), it needs to return one of these Ordering variants to in-
        // dicate how two values compare.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// [Note]
// The guessing game program above follows several common design patterns and Rust
// idioms:
// -----------------------------------------------------------------------------
// 1. Event Loop Pattern - The `loop { ... }` construct creates a continuous
//    interaction cycle that processes user input until a specific condition is met
//    (guessing the correct number).
//
// 2. Pattern Matching with `match` - The program uses Rust's pattern matching
//    in two key places:
//    - To handle the `Result` from parsing the user's input
//    - To evaluate the `Ordering` comparison between the guess and secret number
//
// 3. Error Handling with Result - The program uses the Result type's `match`
//    pattern to handle potential errors when parsing the string input to a number.
//
// 4. Graceful Error Recovery - When invalid input is entered, the program uses
//    `continue` to skip the rest of the loop iteration rather than crashing.
//
// 5. RAII (Resource Acquisition Is Initialization) - Memory management is
//    handled through Rust's ownership system, with resources being automatically
//    cleaned up when they go out of scope.
//
// 6. Immutability by Default - The program follows Rust's principle of immutability
//    by default, only marking variables as mutable (`mut`) when they
//    need to change.
//
// 7. Type Conversion Pattern - The program shows Rust's explicit type conversion
//    with `parse()` to convert from a string to an integer.
//
// This simple program effectively demonstrates fundamental Rust patterns and
// practices in a concise example.
