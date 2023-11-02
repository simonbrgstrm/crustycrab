extern crate chrono;

pub fn checkout_time() {
    // Implement the check-in logic here
    println!("Checked out at: {:?}", chrono::Local::now());
}
