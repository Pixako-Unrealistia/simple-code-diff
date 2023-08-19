fn main() {
    println!("{:?}", extract_quoted_words("*SADwdasda* SASD ASDa *SDWEA*"));
}

fn extract_quoted_words(input: &str) -> Vec<String>{
    let sepword: Vec<&str> = input.split_whitespace().collect();
    let mut output = Vec::<String>::new();
    if sepword.len() >= 1{
        for i in 0..sepword.len(){
            let sepchar: Vec<char> = sepword[i].chars().collect();
            if sepchar[0] == '*' && sepchar[sepchar.len()-1] == '*'{
                if sepchar.len() > 2{
                    let mut temp = sepchar[1].to_string();
                    for j in 2..sepchar.len() - 1{
                        temp = temp + &sepchar[j].to_string()
                    }
                    output.push(temp)
                }else if sepchar.len() == 2 {
                    output.push("".to_string())
                }
            } 
        }
    }
    output
}

#[test]
fn test_extract_quoted_words() {
assert_eq!(extract_quoted_words(""), Vec::<String>::new());
assert_eq!(extract_quoted_words("C ** *C++* *Java *Python* Rust*"),["", "C++", "Python"]);
}

fn extract_quoted_words_r(input: &str) -> Vec<String> {
    fn extract_recursive(words: &[&str], mut output: Vec<String>) -> Vec<String> {
        if let Some(word) = words.first() {
            let sepchar: Vec<char> = word.chars().collect();
            if sepchar.len() >= 2 && sepchar[0] == '*' && sepchar[sepchar.len() - 1] == '*' {
                if sepchar.len() > 2 {
                    let temp: String = sepchar[1..sepchar.len() - 1].iter().collect();
                    output.push(temp);
                } else {
                    output.push("".to_string());
                }
            }
            extract_recursive(&words[1..], output)
        } else {
            output
        }
    }

    let sepword: Vec<&str> = input.split_whitespace().collect();
    extract_recursive(&sepword, Vec::new())
}

#[test]
fn test_extract_quoted_words_r() {
assert_eq!(extract_quoted_words_r(""), Vec::<String>::new());
assert_eq!(extract_quoted_words_r("C ** *C++* *Java *Python* Rust*"),["", "C++", "Python"]);
}