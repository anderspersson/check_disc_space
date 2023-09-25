use fs2::free_space;
use std::env;
use std::process;

    fn get_size(var: u64, t: &str) -> u64
{
    match t
    {
    "B" => var,
    "K" => var / 1024,
    "M" => var / 1_048_576,
    "G" => var / 1_073_741_824,
    _ => 0,
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut size_type = "G";
    let mut drive = "C:\\";
    let mut verbose = "NO";
 
    if args.len() == 1 {
        println!("Usage: check_disc_space < Sizetype B|K|M|G > <drive/path> <verbose YES>");
        process::exit(0);
    }

    if args.len() > 2 {
        size_type = &args[1];
        drive = &args[2];
    }

    if args.len() > 3 {
        verbose = &args[3];
    }

    let result = free_space(drive);
    // Don't handling Erros => let val = result.unwrap();

    match result {
        Ok(val) => {
            if verbose == "NO" {
                println!("Space was {} {}B",get_size(val,size_type),size_type);
            } else {
                println!("{}",get_size(val,size_type));
            }
        },
        Err(err) => {
            println!("Error {:?}",err);
        }
    }

}
