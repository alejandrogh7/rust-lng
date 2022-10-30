use regex::Regex;

fn main() {
    // Regex
    let regex_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let _regex_times = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    // User Data
    println!("INSERT MATH EXPRESSION >> ");
    let mut expression: String = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // Validations
    //times
    loop {
        let _regex_caps = _regex_times.captures(expression.as_str());
        //debug var's
        //println!("{:?}", _regex_caps);

        if _regex_caps.is_none() {
            break;
        }

        let _regex_caps = _regex_caps.unwrap();

        let cap_expression = _regex_caps.get(0).unwrap().as_str();

        let left_value: i32 = _regex_caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = _regex_caps.get(2).unwrap().as_str().parse().unwrap();

        let times: i32 = left_value * right_value;

        expression = expression.replace(cap_expression, &times.to_string());
    }
    //sum
    loop {
        let _regex_caps = regex_add.captures(expression.as_str());
        //debug var's
        //println!("{:?}", _regex_caps);

        if _regex_caps.is_none() {
            break;
        }

        let _regex_caps = _regex_caps.unwrap();

        let cap_expression = _regex_caps.get(0).unwrap().as_str();

        let left_value: i32 = _regex_caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = _regex_caps.get(2).unwrap().as_str().parse().unwrap();

        let sum: i32 = left_value + right_value;

        expression = expression.replace(cap_expression, &sum.to_string());
    }
    // Show Result
    println!("Result: {}", expression);
}
