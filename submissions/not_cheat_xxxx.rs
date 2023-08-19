fn main() {
    println!("Hello, world!");
}


fn count_vowels(str: &str) -> i64 {
    let vowels = "aeiouAEIOU"; 
    let mut v = 0;
    for c in str.chars() {
        if vowels.contains(c) {
            v += 1;
        }
    }
    v
}

#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
}

fn count_vowels_r (str: &str) -> i64 {
    if str.is_empty() {
        return 0;
    }
    let vowels = "aeiouAEIOU";
    let count = count_vowels_r(&str[1..]);
    if vowels.contains(str.chars().next().unwrap()) {
        return count + 1
    } else {
        return count
    }
}

#[test]
fn test_vowels_count_r() {
    assert_eq!(count_vowels_r(""), 0);
    assert_eq!(count_vowels_r("abEcd"), 2);
    assert_eq!(count_vowels_r("ab12Exey5 7x8U3y5z"), 4);
}

fn count_vowels_v2(input: &str) -> Vec<(String,usize)> {
    let mut results = Vec::new();
    let vowels = "aeiouAEIOU";

    for w in input.split_whitespace() {
        let count = w.chars().filter(|c| vowels.contains(*c)).count();
        results.push((w.to_string(), count));
    }
    results
}

#[test]
fn test_vowels_count2() {
assert_eq!(count_vowels_v2(""), []);
assert_eq!(
count_vowels_v2("ab12Exey5 7x8U3y5z"),
[("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
("7x8U3y5z".to_string(), 1) // 'U'
]);
}

fn split_grades(grades: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut okay = Vec::new();
    let mut fail = Vec::new();

    for grade in grades {
        match grade {
            "A+" | "A" | "A-" | "B+" | "B" | "B-" | "C+" | "C" | "C-" => okay.push(grade),
            _ => fail.push(grade),
        }
    }
    
    return (okay, fail)
}

#[test]
fn test_split_grades() {
    assert_eq!(split_grades(vec!["B", "F", "A+", "D", "C"]),
    (vec!["B", "A+", "C"],vec!["F", "D"]));
    assert_eq!(
        split_scores(vec![42, 54]),(vec![], vec![("F".to_string(), 42), ("D".to_string(), 54)]));
    assert_eq!(
        split_scores(vec![75, 98, 63]),(vec![("B".to_string(), 75),("A+".to_string(), 98),("C".to_string(), 63)], vec![]));
}

fn split_scores(scores: Vec<i64>) -> (Vec<(String, i64)>, Vec<(String, i64)>) {
    let mut okay = Vec::new();
    let mut fail = Vec::new();

    for x_int in scores {
        let grade = if (0 <= x_int) && (x_int <= 49) {
            "F".to_string()
        } else if (50 <= x_int) && (x_int <= 60) {
            "D".to_string()
        } else if (61 <= x_int) && (x_int <= 70) {
            "C".to_string()
        } else if (71 <= x_int) && (x_int <= 80) {
            "B".to_string()
        } else if (81 <= x_int) && (x_int <= 90) {
            "A".to_string()
        } else if (95 <= x_int) && (x_int <= 100) {
            "A+".to_string()   
        } else if x_int < 0 {
            "Invalid score".to_string()
        } else if x_int >= 100 {
            "Invalid score".to_string()
        } else {
            "Error".to_string()
        };
        
        if grade == "D" || grade == "F" {
            fail.push((grade, x_int));
        } else {
            okay.push((grade, x_int));
        }
    }

    (okay, fail)
}

#[test]
fn test_split_scores() {
    assert_eq!(split_scores(vec![75, 42, 98, 54, 63]),(vec![("B".to_string(), 75),("A+".to_string(), 98),("C".to_string(), 63)],
            vec![("F".to_string(), 42), ("D".to_string(), 54)]));
    assert_eq!(
        split_scores(vec![42, 54]),(vec![], vec![("F".to_string(), 42), ("D".to_string(), 54)]));
    assert_eq!(
        split_scores(vec![75, 98, 63]),(vec![("B".to_string(), 75),("A+".to_string(), 98),("C".to_string(), 63)], vec![]));
}

fn extract_quoted_words(input: &str) -> Vec<String> {
    let mut quote_word = Vec::new();
    let words: Vec<&str> = input.split_whitespace().collect();

    for word in words {
        let chars: Vec<char> = word.chars().collect();

        if chars.len() > 2 {
            if chars[0] == '*' && chars[chars.len() - 1] == '*' {
                let extracted = &word[1..word.len() - 1];
                quote_word.push(extracted.to_string());
            } 
        }else if word == "**" {
                quote_word.push("".to_string());
            }
    }

    quote_word
}

#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<String>::new());
    assert_eq!(extract_quoted_words("C ** *C++* *Java *Python* Rust*"),vec!["", "C++", "Python"]);
    assert_eq!(extract_quoted_words_r("*****"),vec!["***"]);
}

fn extract_quoted_words_r(input: &str) -> Vec<String> {
    let mut quote_word = Vec::new();
    let mut words = input.split_whitespace();

    if let Some(word) = words.next() {
        if word.starts_with('*') && word.ends_with('*') {
            let extracted = &word[1..word.len() - 1];
            quote_word.push(extracted.to_string());
        } else if word == "**" {
            quote_word.push("".to_string());
        }

        quote_word.extend(extract_quoted_words_r(words.collect::<Vec<&str>>().join(" ").as_str()));
    }

    quote_word
}

#[test]
fn test_extract_quoted_words_r() {
    assert_eq!(extract_quoted_words_r(""), Vec::<String>::new());
    assert_eq!(extract_quoted_words_r("C ** *C++* *Java *Python* Rust*"),vec!["", "C++", "Python"]);
    assert_eq!(extract_quoted_words_r("*****"),vec!["***"]);
}
