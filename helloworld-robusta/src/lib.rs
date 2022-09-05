use robusta_jni::bridge;

#[bridge]
mod jni {
    use robusta_jni::convert::Signature;

    #[derive(Signature)]
    #[package()] // default package
    struct HelloWorldRobusta;

    impl HelloWorldRobusta {
        pub extern "jni" fn hello(input: String) -> String {
            format!("Hello, {}!", input)
        }
    }
}