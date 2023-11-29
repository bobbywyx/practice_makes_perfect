use editor::run;

fn main() {
    if let Err(e) = run(){
        println!("{}",e);
        std::process::exit(1);
    }
}

