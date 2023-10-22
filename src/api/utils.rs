use rand::Rng;

fn generate_random_string(length: usize) -> Vec<char> {
    let possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let possible_chars: Vec<char> = possible.chars().collect();

    vec![0; length]
        .iter()
        .map(|_| {
            let random_index = rng.gen_range(0..possible_chars.len());
            possible_chars[random_index]
        })
        .collect()
}
