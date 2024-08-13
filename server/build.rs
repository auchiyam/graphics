fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../api/proto/helloworld.proto")?;
    tonic_build::compile_protos("../api/proto/renderer.proto")?;
    Ok(())
}
