// SPDX-License-Identifier: Apache-2.0

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());

    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["../../specs/securefabric.proto"], &["../../specs"])?;
    Ok(())
}
