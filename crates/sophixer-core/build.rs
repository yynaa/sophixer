use std::io::Result;
fn main() -> Result<()> {
  // prost_build::compile_protos(
  //   &[
  //     "../../protobuf/renoise_from.proto",
  //     "../../protobuf/renoise_to.proto",
  //   ],
  //   &["../../protobuf/"],
  // )?;

  prost_build::Config::new()
    .type_attribute(".", "#[derive(derive_more::From)]")
    .compile_protos(
      &[
        "../../protobuf/renoise_from.proto",
        "../../protobuf/renoise_to.proto",
      ],
      &["../../protobuf/"],
    )?;

  Ok(())
}
