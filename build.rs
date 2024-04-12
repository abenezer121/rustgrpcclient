

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
       .type_attribute(".routingpackage.Location", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".routingpackage.ONMDirectionArrData" , "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["proto/routing.proto"], &["."])?;
    Ok(())
}
