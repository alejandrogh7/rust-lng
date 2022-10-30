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

    //get age
    println!("Introduce your age >> ");
    let mut user_age: String = String::new();
    std::io::stdin().read_line(&mut user_age).unwrap();

    //convert into integer
    let _user_age_int: u8 = user_age.trim().parse().unwrap();

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

}
