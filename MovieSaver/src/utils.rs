use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Local;

/// Struct representing movie information.
/// Each movie has a timestamp, title, release year, and price.
#[derive(Debug, Clone)]
pub struct MovieInfo {
    pub timestamp: String,
    pub title: String,
    pub year: i32,
    pub price: f64,
}

/// Returns the current timestamp as a formatted string.
/// Uses the local time in "YYYY-MM-DD HH:MM:SS" format.
pub fn get_current_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Ensures the "MovieData" directory exists for storing movie files.
/// If the directory does not exist, it will be created.
/// Prints an error message if directory creation fails.
pub fn ensure_movie_directory_exists() {
    let dir = Path::new("MovieData");
    if !dir.exists() {
        if let Err(e) = fs::create_dir(dir) {
            eprintln!("Error creating directory: {}", e);
        }
    }
}

/// Prints the details of a single movie to the console.
pub fn display_movie(movie: &MovieInfo) {
    println!("Title: {}", movie.title);
    println!("Year: {}", movie.year);
    println!("Price: ${:.2}", movie.price);
    println!("Last Updated: {}", movie.timestamp);
}

/// Prompts the user to input movie details (title, year, price).
/// Returns a MovieInfo struct with the entered data and current timestamp.
pub fn input_movie() -> MovieInfo {
    let mut title = String::new();
    let mut year = String::new();
    let mut price = String::new();

    println!("Enter movie title: ");
    io::stdin().read_line(&mut title).expect("Failed to read title");
    let title = title.trim().to_string();

    println!("Enter release year: ");
    io::stdin().read_line(&mut year).expect("Failed to read year");
    let year: i32 = year.trim().parse().unwrap_or(0);

    println!("Enter current price: $");
    io::stdin().read_line(&mut price).expect("Failed to read price");
    let price: f64 = price.trim().parse().unwrap_or(0.0);

    MovieInfo {
        timestamp: get_current_timestamp(),
        title,
        year,
        price,
    }
}

/// Saves the list of movies to a file in the "MovieData" directory.
/// Each movie is written as a line with fields separated by '|'.
/// Prints an error message if the file cannot be created or written.
pub fn save_movies(movies: &[MovieInfo], filename: &str) {
    ensure_movie_directory_exists();
    let path = Path::new(filename);
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to save movies to {}: {}", filename, e);
            return;
        }
    };

    for movie in movies {
        if let Err(e) = writeln!(
            file,
            "{}|{}|{}|{}",
            movie.timestamp, movie.title, movie.year, movie.price
        ) {
            eprintln!("Failed to write movie: {}", e);
        }
    }
}

/// Loads movies from a file in the "MovieData" directory.
/// Each line is parsed into a MovieInfo struct.
/// Returns a vector of loaded movies. If the file does not exist, returns an empty vector.
pub fn load_movies(filename: &str) -> Vec<MovieInfo> {
    let path = Path::new(filename);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    let mut movies = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 4 {
                let year = parts[2].parse().unwrap_or(0);
                let price = parts[3].parse().unwrap_or(0.0);
                movies.push(MovieInfo {
                    timestamp: parts[0].to_string(),
                    title: parts[1].to_string(),
                    year,
                    price,
                });
            }
        }
    }
    movies
}

/// Displays all movies in the provided list.
/// If the list is empty, prints a message indicating no movies are present.
pub fn display_all_movies(movies: &[MovieInfo]) {
    if movies.is_empty() {
        println!("No movies in database.");
        return;
    }
    println!("\n=== Movie Database ===");
    for movie in movies {
        display_movie(movie);
        println!("---------------------");
    }
}

/// Prompts the user to delete a movie by its index.
/// Removes the selected movie from the list if the index is valid.
pub fn delete_movie(movies: &mut Vec<MovieInfo>) {
    if movies.is_empty() {
        println!("No movies to delete.");
        return;
    }
    println!("\n=== Delete a Movie ===");
    for (i, movie) in movies.iter().enumerate() {
        println!("{}. {} ({})", i + 1, movie.title, movie.year);
    }
    print!("Enter the number of the movie to delete: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let idx: usize = match input.trim().parse() {
        Ok(num) if num > 0 && num <= movies.len() => num - 1,
        _ => {
            println!("Invalid selection.");
            return;
        }
    };
    let removed = movies.remove(idx);
    println!("Deleted \"{}\" ({})", removed.title, removed.year);
}

/// Main Movie Database function.
/// Handles the user interface loop for adding, viewing, and saving movies.
/// Returns Ok(0) on normal exit, or Err(String) on error.
pub fn run_movie_db() -> Result<i32, String> {
    let filename = "MovieData/movies.txt";
    let mut movies = load_movies(filename);

    loop {
        // Display menu options
        println!("\nMovie Database Menu:");
        println!("1. Add new movie");
        println!("2. View all movies");
        println!("3. Delete a movie");
        println!("4. Save & Exit");
        print!("Choice: ");
        io::stdout().flush().unwrap();

        // Read user menu choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");
        let choice = choice.trim();

        match choice {
            "1" => {
                // Add a new movie
                let movie = input_movie();
                movies.push(movie);
            }
            "2" => {
                // Display all movies
                display_all_movies(&movies);
            }
            "3" => {
                // Delete a movie
                delete_movie(&mut movies);
            }
            "4" => {
                // Save movies and exit
                save_movies(&movies, filename);
                println!("Data saved. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
    Ok(0)
}