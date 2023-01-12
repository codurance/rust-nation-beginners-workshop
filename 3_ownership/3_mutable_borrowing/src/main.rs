fn main() {
    let mut ingredients_to_ban_from_pizza = String::new();

    ban(&ingredients_to_ban_from_pizza, "pineapple");
    ban(&ingredients_to_ban_from_pizza, "anchovies");
    ban(&ingredients_to_ban_from_pizza, "the collected works of Leo Tolstoy");

    print_banned_ingredients(&mut ingredients_to_ban_from_pizza);
}

fn ban(ingredients_to_ban_from_pizza: String, ingredient: &str) {
    ingredients_to_ban_from_pizza.push_str(ingredient);
    ingredients_to_ban_from_pizza.push_str(", ");
}

fn print_banned_ingredients(ingredients_to_ban_from_pizza: &mut String) {
    println!("{}", ingredients_to_ban_from_pizza);
}
