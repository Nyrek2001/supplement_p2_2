#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_normal_random() {
        let mean = 0.0;
        let std_dev = 1.0;
        let result = generate_normal_random(mean, std_dev);
        assert!(result >= -3.0 && result <= 3.0, "Result should be within 3 standard deviations.");
    }

    #[test]
    fn test_generate_password() {
        let length = 10;
        let allowed_chars = vec!['a', 'b', 'c', 'd', '1', '2', '3'];
        let password = generate_password(length, &allowed_chars);
        assert_eq!(password.len(), length, "Password length should match the specified length.");
        for ch in password.chars() {
            assert!(allowed_chars.contains(&ch), "Password contains invalid character.");
        }
    }

    #[test]
    fn test_simulate_dice_roll() {
        let result = simulate_dice_roll();
        assert!(result == 1 || result == -1 || result == 0, "Result should be either 1, -1, or 0.");
    }
}
