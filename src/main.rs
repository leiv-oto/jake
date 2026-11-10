use std::process;
enum Activity {
    New,
    Run,
}

fn main() {
    let activity = Activity::Run;
    let name = "hikoko";
    let mode = "kotlin";
    //Al rato implemento clap
    match activity {
        Activity::New => {

        }
        Activity::Run => {
            process::exit(0)
        }
    }
}