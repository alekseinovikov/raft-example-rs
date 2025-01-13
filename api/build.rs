use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path();
    match protoc {
        Ok(path) => {
            env::set_var("PROTOC", path);
            tonic_build::configure()
                .compile_protos(&["src/api.proto"], &["src"])?;
        }
        Err(err) => {
            eprintln!("protoc not found: {}", err);
            return Err(Box::new(err));
        },
    }

    Ok(())
}
