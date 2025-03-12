use rand::rng;
use rand_distr::{Normal, distribution};

/// Generates a random floating-point number that follows a normal distribution with the given mean and standard deviation.
/// 
/// # Arguments
/// 
/// * `mean` - The mean value of the normal distribution.
/// * `std_dev` - The standard deviation of the normal distribution.
/// 
/// # Returns
/// 
/// A random floating-point number sampled from the normal distribution.
fn generate_normal_random(mean: f64, std_dev: f64) -> f64 {
    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rand::thread_rng();
    normal.sample(&mut rng)
}

/// Generates a random password of the given length using characters from the allowed set.
/// 
/// # Arguments
/// 
/// * `length` - The length of the generated password.
/// * `allowed_chars` - A list of allowed characters to pick from for the password.
/// 
/// # Returns
/// 
/// A string representing the randomly generated password.
fn generate_password(length: usize, allowed_chars: &[char]) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| *allowed_chars.choose(&mut rng).unwrap())
        .collect()
}

/// Simulates two players rolling a 6-sided die and returns:
/// - 1 if player 1 wins,
/// - -1 if player 2 wins,
/// - 0 if it’s a draw.
/// 
/// # Returns
/// 
/// An integer representing the result of the dice roll (1, -1, or 0).
fn simulate_dice_roll() -> i32 {
    let mut rng = rand::thread_rng();
    let player1_roll = rng.gen_range(1..=6);
    let player2_roll = rng.gen_range(1..=6);
    
    if player1_roll > player2_roll {
        1 // Player 1 wins
    } else if player2_roll > player1_roll {
        -1 // Player 2 wins
    } else {
        0 // Draw
    }
}
