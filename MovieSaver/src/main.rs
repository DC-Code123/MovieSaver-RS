// Entry point for the Movie Database Management System application.

mod utils;

fn main() {
    // Print the application header
    println!("=== Movie Database Management System ===");

    // Use catch_unwind to handle unexpected panics gracefully.
    // This is similar to a top-level try-catch in C++.
    let result = std::panic::catch_unwind(|| {
        // Call the main logic function from the utils module.
        // This function handles the user interface and all movie operations.
        utils::run_movie_db()
    });

    // Handle the result of the main logic function and any panics.
    match result {
        // Program ended successfully (returned 0)
        Ok(Ok(0)) => {
            // No action needed, exit with 0
        }
        // Program ended with a non-zero error code
        Ok(Ok(code)) => {
            eprintln!("Program ended with error code: {}", code);
            std::process::exit(code);
        }
        // The main logic function returned an error (as a String)
        Ok(Err(e)) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        // A panic occurred (unhandled error)
        Err(_) => {
            eprintln!("Error: An unexpected error occurred.");
            std::process::exit(1);
        }
    }
}