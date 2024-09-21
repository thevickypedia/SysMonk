#[actix_rt::main]
async fn main() {
    match sysmonk::start().await {
        Ok(_) => {
            println!("\nSysMonk session has ended")
        }
        Err(err) => {
            eprintln!("\nError starting SysMonk: {}", err)
        }
    }
}
