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

/*
Passing `job` to `take_job` moves the ownership of the String into the
function. A String owns heap-allocated text, so moving it transfers
ownership but does not make a new copy of that text. When the function
returns, its String is dropped and the heap allocation is freed, so Rust
prevents us from using `job` afterward because it no longer owns a valid
String.

An i32 implements Copy, so passing `worker_limit` copies its value to the
function. And so, both the original and the copy remain independent and
usable. Unlike String, i32 has no heap allocation of its own that needs
to be freed.

If take_job only needed to read the description, it could accept &str
and borrow it instead. That would also let the caller keep using job
afterward. Calling job.clone() would also preserve the original, but it
would also create a separate owned copy of the string.
*/
