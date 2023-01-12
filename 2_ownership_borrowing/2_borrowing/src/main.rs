fn main() {
    let members_of_iron_maiden = String::from("Steve Harris,Bruce Dickinson,Dave Murray,Nicko McBrain,Adrian Smith");

    let members_of_iron_maiden = print_singer(members_of_iron_maiden);
    print_bassist(members_of_iron_maiden);
}

fn print_singer(members_of_iron_maiden: String) -> String {
    println!("Singer: {}", &members_of_iron_maiden[13..28]);
    members_of_iron_maiden
}

fn print_bassist(members_of_iron_maiden: String) -> String {
    println!("Bassist: {}", &members_of_iron_maiden[0..12]);
    members_of_iron_maiden
}
