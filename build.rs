use duckscript::{runner, types::runtime::Context};
use std::{env, fs, path::PathBuf};

fn main() {
    let mut context = Context::new();
    duckscriptsdk::load(&mut context.commands).expect("Error setting up duckscript environment.");
    runner::run_script(
        r#"
    root = get_env CARGO_MANIFEST_DIR
    target = get_env OUT_DIR
    
    if not is_path_exists ${root}/target/ember-plus
        exec git clone https://github.com/Lawo/ember-plus.git ${root}/target/ember-plus
    else
        echo ${root}/target/ember-plus already exists.
    end
    cd ${root}/target/ember-plus
    exec git config advice.detachedHead false
    exec git checkout v1.8.2.1
    cd ${target}
    if not is_path_exists ${target}/libember_slim/libember_slim-shared.so
        exec cmake ${root}/target/ember-plus
        exec make
    else
        echo ${target}/libember_slim/libember_slim-shared.so already exists.
    end
    cd ${root}
    "#,
        context,
    )
    .expect("Compiling Ember+ failed!");

    let root = env::var("CARGO_MANIFEST_DIR").expect("failed to get manifest dir");
    let out = env::var("OUT_DIR").expect("failed to get out dir");

    // Tell cargo to tell rustc to link the ember_slim-shared library.
    println!("cargo:rustc-link-search=native={}/libember_slim/", out);
    match env::var("CARGO_FEATURE_STATIC") {
        Ok(_) => println!("cargo:rustc-link-lib=static=ember_slim-static"),
        _ => println!("cargo:rustc-link-lib=ember_slim-shared"),
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(format!(
            "{}/target/ember-plus/libember_slim/Source/emberplus.h",
            root
        ))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(env::var("CARGO_FEATURE_DOC").is_ok())
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(out).join("generated");
    if !out_path.is_dir() {
        fs::create_dir(&out_path).expect("could not create output directory");
    }
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
