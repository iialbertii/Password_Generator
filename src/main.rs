const CHARACTERS_FOR_GEN: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*:,./";

fn main() {
    let rng_len: u32 = rand::random_range(5..20);
    let mut password: String = String::new();
    let characters: Vec<char> = CHARACTERS_FOR_GEN.chars().collect();
    for _ in 0..rng_len {
        let rng_index_from_set = rand::random_range(0..characters.len());
        password.push(characters[rng_index_from_set]);
    }
    println!("Password :{}", password);
}
