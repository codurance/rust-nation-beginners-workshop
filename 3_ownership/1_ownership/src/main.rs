fn main() {
    let facilitator = get_facilitator();
    let capitalised = capitalise(facilitator);

    log_event(capitalised);

    println!("{} likes pizza", capitalised);
}

fn get_facilitator() -> String {
    String::from("mattsi")
}

fn capitalise(input: String) -> String {
    input.to_uppercase()
}

fn log_event(input: String) {
    //send `input` to an IO or HTTP event
}