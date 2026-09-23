# CS230H Assignment 1

## Project idea

A simple job scheduler that processes jobs in order. Over the semester, I plan to add a linked job queue, multiple worker threads, and a server that accepts jobs from network clients.

## Run

```sh
cargo run
```

## Ownership and copying

The program passes a job description (`String`) to `take_job` and a worker limit (`i32`) to `show_worker_limit`.

- The `String` moves into `take_job`. The original variable cannot be used afterward, and the string's memory is freed when the function ends.
- The `i32` implements `Copy`, so the function receives a copy. The original worker limit can still be used afterward.

Uncommenting the second print of `job` produces a "borrow of moved value" compiler error. Leaving that line commented out lets the program compile and run.
