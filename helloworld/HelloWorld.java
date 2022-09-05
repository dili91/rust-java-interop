public class HelloWorld {

    static {
        System.loadLibrary("hello_world");
    }
    private static native String hello(String name);

    public static void main(String... args){
        String formatted = hello(args[0]);
        System.out.println(formatted);
    }
}
