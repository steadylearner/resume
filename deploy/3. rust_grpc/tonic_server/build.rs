// Cargo will pick up on the existence of this file, then compile and execute it before the rest of the crate is built. This can be used to generate code at compile time. 

// https://docs.rs/prost-build/0.3.2/prost_build/struct.Config.html#examples-2
// https://docs.rs/tonic-build/0.1.0-beta.1/tonic_build/struct.Builder.html
// https://github.com/hyperium/tonic/tree/master/tonic-build#configuration

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It will work with src(_manually_implementing_serde)
    tonic_build::compile_protos("proto/user/user.proto").unwrap();
    Ok(())

    // It should work with src_not_manually_implementing_serde
    // (It currently shows that Serialize and Deserialize macro defintion are not availiable)

    //     error: cannot find derive macro `Serialize` in this scope
    // --> target/debug/build/user-grpc-f29092ef26d7f485/out/user.rs:8:46
    // |
    // 8 | #[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize, Debug)]
    // |                                              ^^^^^^^^^

    // error: cannot find derive macro `Deserialize` in this scope
    // --> target/debug/build/user-grpc-f29092ef26d7f485/out/user.rs:8:57
    // |
    // 8 | #[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize, Debug)]

    //   -> Read more documentation or ask for help of the author.
    // tonic_build::configure()
    //     // It is included in the out/user.rs but the compiler says it can not find them.
    //     .type_attribute(".user.UserReply", "#[derive(Serialize, Deserialize, Debug)]")
    //     .compile(
    //         &["proto/user/user.proto"],
    //         &["proto/user"]
    //     )?;
    // Ok(())
}
