fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn next_prime(current_prime: u32) -> u32 {
    let mut guess = current_prime + 1;
    while !is_prime(guess) {
        guess += 1;
    }
    guess
}

pub fn previous_prime(current_prime: u32) -> u32 {
    let mut guess = current_prime - 1;
    while !is_prime(guess) {
        guess -= 1;
    }
    guess
}
