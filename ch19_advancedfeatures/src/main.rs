mod functions;
mod lifetimes;
mod traits;
mod types;
mod unsafe_demo;

fn main() {
    unsafe_demo::demo();
    lifetimes::demo();
    traits::demo();
    types::demo();
    functions::demo();
}
