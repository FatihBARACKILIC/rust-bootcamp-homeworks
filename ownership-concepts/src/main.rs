fn main() {
    let string1 = String::from("string1");
    let string2 = String::from("string2");

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);
}

fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut new_string: String = String::new();

    new_string.push_str(str1);
    new_string.push_str(str2);

    new_string
}
