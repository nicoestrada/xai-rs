fn main() {
    tonic_build::configure()
        .compile_protos(&["proto/xai/api/v1/chat.proto"], &["proto"])
        .unwrap();
}
