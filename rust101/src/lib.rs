pub fn foo() {
    let word = String::from("qwe_qwe qwe qwedas d");
    first_word(&word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
