fn take_job(job: String) {
    println!("Job: {job}");
}

fn show_worker_limit(limit: i32) {
    println!("Worker limit: {limit}");
}

fn main() {
    let job = String::from("Generate a report");
    take_job(job);

    // Uncomment this to see what happens after moving the String:
    // println!("Job after the call: {job}");

    let worker_limit = 2;
    show_worker_limit(worker_limit);
    println!("Worker limit after the call: {worker_limit}");
}
