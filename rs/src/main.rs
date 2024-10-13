use std::{
    env, error::Error, process
};
use wasmtime::{
    Engine, Linker, Module, Store
};
use wasmtime_wasi::{
    preview1::{self, WasiP1Ctx}, DirPerms, FilePerms, WasiCtxBuilder
};

const WASI_BINARY: &[u8] = include_bytes!("lenv-wasip1-0.1.7.wasm");

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();

    let module = match Module::from_binary(&engine, WASI_BINARY) {
        Ok(m) => m,
        Err(_) => {
            eprintln!("lenv: failed to load wasi binary");
            process::exit(1);
        }
    };

    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    match preview1::add_to_linker_sync(&mut linker, |t| t) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("lenv: failed to add wasip1 functions to linker");
            process::exit(1);
        }
    }
    let pre = match linker.instantiate_pre(&module) {
        Ok(p) => p,
        Err(_) => {
            eprintln!("lenv: failed to instantiate module with linker");
            process::exit(1);
        }
    };

    let cwd = match env::current_dir() {
        Ok(c) => c,
        Err(_) => {
            eprintln!("lenv: failed to get current working directory");
            process::exit(1);
        }
    };
    let cwd_str = match cwd.to_str() {
        Some(c) => c,
        None => {
            eprintln!("lenv: failed to convert current working directory to string");
            process::exit(1);
        }
    };

    let mut wasi_builder = WasiCtxBuilder::new();
    wasi_builder.inherit_args();
    wasi_builder.inherit_stdio();
    match wasi_builder.preopened_dir(&cwd, cwd_str, DirPerms::all(), FilePerms::all()) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("lenv: failed to preopen current working directory");
            process::exit(1);
        }
    };
    let wasi = wasi_builder.build_p1();

    let mut store = Store::new(&engine, wasi);
    let instance = match pre.instantiate(&mut store) {
        Ok(i) => i,
        Err(_) => {
            eprintln!("lenv: failed to create new instance in store");
            process::exit(1);
        }
    };

    let wasi_start_func = "_start";
    let start = match instance.get_func(&mut store, wasi_start_func) {
        Some(f) => f,
        None => {
            eprintln!("lenv: wasi {} function not found", wasi_start_func);
            process::exit(1);
        }
    };

    let start_typed = match start.typed::<(), ()>(&store) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("lenv: failed to get typed function");
            process::exit(1);
        }
    };

    let result = start_typed.call(&mut store, ());

    match result {
        Ok(_) => (),
        Err(_) => {
            std::process::exit(1);
        }
    }

    Ok(())
}
