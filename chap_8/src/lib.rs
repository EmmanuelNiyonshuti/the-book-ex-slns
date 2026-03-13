use std::collections::HashMap;

// Exercise 1: median and mode from a list of numbers

pub fn find_median(numbers: &mut Vec<u32>) -> u32 {

    numbers.sort();
    let in_middle = numbers.len() / 2;
    let median = &numbers[in_middle];
    *median
}

pub fn find_mode(numbers: &Vec<u32>) -> u32 {
    // we construct a map of number and its frequency `{num: 0}` from the list.
    // and then we get the number from the map with big frequency

    let mut counter_map = HashMap::new();
    for num in numbers {
        *counter_map.entry(*num).or_insert(0) += 1;
    }
    let mut max_frequency = 0;
    let mut mode: u32 = 0;
    for (num, frequency) in counter_map {
        if frequency > max_frequency{
            max_frequency = frequency;
            mode = num;
        }
    }
    mode

}

// Exercise2: Converting a string of text into a pig latin string(pig )
pub fn convert_text_str_to_pig_latin(text: &mut String) -> String{
    let vowels = "aeiouAEIOU";
    let mut result = String::new();
    for word in text.split_whitespace() {
        let first_char = &word[0..1];
        let pig_latin = if vowels.contains(first_char) {
            format!("{word}-hay")
        }else {
            let word_len = word.len();
            let rest = &word[1..word_len];
            format!("{rest}-{first_char}ay")
        };
        result.push_str(&pig_latin);
        result.push(' ');
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_mode(){
        let numbers = vec![100, 200, 32, 498, 81, 32, 67, 71, 50, 74];
        let mode = find_mode(&numbers);
        assert_eq!(mode, 32);
    }
    #[test]
    fn test_find_median(){
        let mut numbers = vec![100, 200, 32, 498, 81, 32, 67, 71, 50, 74];
        let median = find_median(&mut numbers);
        assert_eq!(median, 74);
    }

    #[test]
    fn test_convert_text_str_to_pig_latin() {
        let mut text = String::from("i want the first apple");

        let pig_latin = convert_text_str_to_pig_latin(&mut text);
        assert_eq!(pig_latin, String::from("i-hay ant-way he-tay irst-fay apple-hay"));
    }
}
