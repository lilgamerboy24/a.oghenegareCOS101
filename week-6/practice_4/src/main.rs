fn main() {
    let fullname = "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    println!("My name is: {}", fullname);
    println!("Department: {}", department);
    println!("University: {}", uni);

    // Convert and manipulate strings as shown in the example
    let mut school = "School of Science".to_string();
    // replace "School" with "Faculty" (as an example from the slide)
    school = school.replace("School", "Faculty");
    println!("School after replace: {}", school);

    // Example of using to_string to ensure a String type
    let full_title = format!("{} {}", department, uni);
    println!("Full title: {}", full_title);
}