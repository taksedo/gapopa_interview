use std::env;

use usecase::check_site_health_use_case;

fn main() {
    let args: Vec<_> = env::args().collect();
    let result = check_site_health_use_case(args);
    if result.is_err() {
        eprintln!("{}", result.unwrap_err())
    }
}
