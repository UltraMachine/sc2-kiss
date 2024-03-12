use std::path::Path;

fn main() {
	let protos_dir = Path::new("../s2client-proto/s2clientprotocol");
	println!("cargo:rerun-if-changed={}", protos_dir.display());

	let mut proto_file = protos_dir.to_owned();
	proto_file.push("sc2api.proto");

	prost_build::Config::new()
		.compile_protos(&[proto_file], &[protos_dir.parent().unwrap()])
		.unwrap_or_else(|e| panic!("{e}"))
}
