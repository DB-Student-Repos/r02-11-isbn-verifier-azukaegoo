/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {

    // unimplemented!("Is {isbn:?} a valid ISBN number?");
    let removed_hyphened_isbn: String = isbn.chars().filter(|&c| c != '-').collect();

    if removed_hyphened_isbn.len() != 10 {
        return  false;
    }

    let mut i_sbn = 0;
    for (index, ch) in removed_hyphened_isbn.chars().enumerate() {
        i_sbn += match (index, ch) {
            (9, 'X') => 10,
            (_, '0'..='9') => ch.to_digit(10).unwrap() * (10 - index as u32),
            _ => return false,
        };

    }
    i_sbn % 11 == 0
}
