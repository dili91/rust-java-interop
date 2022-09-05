public class Signing {

    static {
        System.loadLibrary("truelayer_signing_robusta");
    }

    private static native String sign(String kid, String privateKeyLocation,
        String method, String path, String body);

    public static void main(String... args){
        var signature = sign("45fc75cf-5649-4134-84b3-192c2c78e990",
                "./ec512-private.pem",
                "post",
                "/mandates",
                """
                    {"amount":123}        
                        """);

        System.out.println(signature);
    }
}