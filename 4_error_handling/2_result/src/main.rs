fn send_temperature_request(temp: i32) -> Result<(),String> {
    //Simulates sending a web request to set the temperature in an IoT thermometer
    if temp <= 10 {
        Ok(())
    } else {
        Err(String::from("Request to thermometer timed out"))
    }
}

fn log_error(err: String) {
    //Simulates logging an error
    println!("{err}");
}

fn set_temperature(temp: i32) {
    unimplemented!();
}

fn main() {
    set_temperature(10);
    set_temperature(30);
}
