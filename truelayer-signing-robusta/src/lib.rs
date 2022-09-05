use robusta_jni::bridge;

#[bridge]
mod jni {
    use std::fs;
    use robusta_jni::convert::Signature;

    #[derive(Signature)]
    #[package()] // default package
    struct Signing;

    impl Signing {
        pub extern "jni" fn sign(
            kid: String,
            private_key_location: String,
            method: String,
            path: String,
            body: String,
        ) -> String {
            let private_key = fs::read(private_key_location)
                .expect("Unable to read the private key");

            truelayer_signing::sign_with_pem(kid.as_str(), &private_key)
                .method(method.as_str())
                .path(path.as_str())
                .body(body.as_bytes())
                .sign()
                .unwrap()
        }
    }
}
