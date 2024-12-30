mod trishexadecimal;
use trishexadecimal::base36;
use crate::trishexadecimal::base36::Base36;

fn main() {
    let final_number = base36::base36_to_num("hol".to_string());
    let final_base36_number = base36::num_to_base36(final_number);
    println!("{}", final_number);
    println!("{}", final_base36_number);
    let base_test = Base36::new();
    base_test.test();

}

#[cfg(test)]
mod tests {
    use crate::trishexadecimal::base36;
    #[test]
    // Este test debe de fallar si o si
    fn test_base36() {
        let test_numbers = ["123qwsa", "sadad", "cxxzc", "dassa", "asdasd"];
        for test_number in test_numbers {
            println!("test_number: {}", base36::base36_to_num(test_number.to_string()));
        }
    }

    // Este test debe de ejecutarse correctamente
    #[test]
    fn test_trishexadecimal() {
        let test_numbers = ["123", "sad", "cxx", "das", "asd"];
        for test_number in test_numbers {
            println!("test_number: {}", base36::base36_to_num(test_number.to_string()));
        }
    }
}
