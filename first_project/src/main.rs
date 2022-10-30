fn main() {
    let name: &str = "Ale";
    let age: u8 = 19;
    let mut fav_number: i8 = 7;
    fav_number = fav_number + 1;

    println!("Hello I am {} and I am {} years old", name, age);
    println!("Favorite number {}", fav_number);

    println!("Introduce your name >> ");
    //strings simple sin metodos
    //let mut user_name: &str = "";
    //string complejos con metodos
    let mut user_name: String = String::new();
    user_name = user_name.trim().to_string();

    std::io::stdin().read_line(&mut user_name).unwrap();

    println!("Introduce your age >> ");
    let mut user_age: String = String::new();
    std::io::stdin().read_line(&mut user_age).unwrap();

    //convert into integer
    let _user_age_int: u8 = user_age.trim().parse().unwrap();

    if _user_age_int >= 18 && (_user_age_int <= 70 || _user_age_int == 80) {
        println!("GO IN");
    } else if _user_age_int == 90 {
        println!("GO ???")
    } else {
        println!("GO OUT")
    }

    println!("Hello, welcome {} your age is {}", user_name, _user_age_int);

    //chllng 1
    let mut new_name: String = String::new();
    let mut new_country: String = String::new();

    println!("Insert name >> ");
    std::io::stdin().read_line(&mut new_name).unwrap();

    println!("Insert country >> ");
    std::io::stdin().read_line(&mut new_country).unwrap();

    new_name = new_name.trim().to_string();
    new_country = new_country.trim().to_string();

    println!("Hello {} from {}", new_name, new_country);

    //chllng 2
    let mut choose: String = String::new();

    println!("Choose either 0 or 1 >> ");
    std::io::stdin().read_line(&mut choose).unwrap();

    let new_choose: i8 = choose.trim().parse().unwrap();

    if new_choose == 0 {
        println!("U L0000000SE");
    } else if new_choose == 1 {
        println!("U W111111N");
    } else {
        println!("BR000 ????");
    }

    let number_1: i32 = 100;
    let number_2: i32 = 10;

    let sum: i32 = number_1 + number_2;
    println!("SUM: {} + {} = {}", number_1, number_2, sum);

    //create loop
    loop {
        let mut sum_user: String = String::new();

        println!("INSERT NUMBER >> ");
        std::io::stdin().read_line(&mut sum_user).unwrap();

        let _sum_user_int: i32 = sum_user.trim().parse().unwrap();

        if sum == _sum_user_int {
            println!("SAME");
            //break loop
            break;
        } else {
            println!("DIFF");
        }
    }
}
