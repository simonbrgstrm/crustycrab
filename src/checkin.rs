extern crate chrono;

pub fn checkin_time() {
    // Implement the check-in logic here
    println!("Checked in at: {:?}", chrono::Local::now());
}
