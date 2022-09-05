public class HelloWorldRobusta {
    static {
        System.loadLibrary("hello_world_robusta");
    }

    private static native String hello(String name);

    public static void main(String[] args) {
        String formattedText = hello(args[0]);
        System.out.println(formattedText);
    }
}
