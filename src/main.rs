use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

static SNAPSHOT0: &str = include_str!("../interfaces/wasi_snapshot_preview0.wasm_int");
static SNAPSHOT1: &str = include_str!("../interfaces/wasi_snapshot_preview1.wasm_int");
static EPHEMERAL: &str = include_str!("../interfaces/wasi_ephemeral_preview.wasm_int");

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// the Wasm file to check
    #[structopt(parse(from_os_str))]
    wasm_file: PathBuf,
    #[structopt(long = "verbose", short = "v")]
    verbose: bool,
}

fn main() {
    let opt = Opt::from_args();
    let bytes = match std::fs::read(&opt.wasm_file) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!(
                "Could not read from file \"{}\": {}",
                opt.wasm_file.to_string_lossy(),
                e.description()
            );
            std::process::exit(-2);
        }
    };

    let snapshot0 = wasm_interface::parser::parse_interface(SNAPSHOT0).expect("internal error");
    let snapshot0_result =
        wasm_interface::validate::validate_wasm_and_report_errors(&bytes, &snapshot0);
    if snapshot0_result.is_ok() {
        println!("WASI snapshot preview 0 (a.k.a. \"wasi_unstable\") found!");
        std::process::exit(0);
    }

    let snapshot1 = wasm_interface::parser::parse_interface(SNAPSHOT1).expect("internal error");
    let snapshot1_result =
        wasm_interface::validate::validate_wasm_and_report_errors(&bytes, &snapshot1);
    if snapshot1_result.is_ok() {
        println!("WASI snapshot preview 1 found!");
        std::process::exit(0);
    }

    let ephemeral = wasm_interface::parser::parse_interface(EPHEMERAL).expect("internal error");
    let ephemeral_result =
        wasm_interface::validate::validate_wasm_and_report_errors(&bytes, &ephemeral);
    if ephemeral_result.is_ok() {
        println!("WASI ephemeral found!");
        std::process::exit(0);
    }
    eprintln!("No WASI version identified.  Please update to the latest version of this tool and try again.");
    if opt.verbose {
        if let Err(e) = snapshot0_result {
            eprintln!("Snapshot 0 errors: {:#?}", e);
        }
        if let Err(e) = snapshot1_result {
            eprintln!("Snapshot 1 errors: {:#?}", e);
        }
        if let Err(e) = ephemeral_result {
            eprintln!("Ephemeral errors: {:#?}", e);
        }
    } else {
        eprintln!("\nRun with -v to see errors");
    }

    std::process::exit(-1);
}
