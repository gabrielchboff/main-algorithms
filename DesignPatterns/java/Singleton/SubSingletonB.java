package Singleton;

/**
 *  A real Singleton we want to have
 */
public class SubSingletonB extends SingletonB {
    public static boolean instanceFlag = false; //true if 1 instance
    //private int i = 0;
    
    public SubSingletonB() {
        if(instanceFlag) {
            System.out.println("Only can create a instance !");
        } else {
            instanceFlag = true;
            super.Register("Sub1", this);
        }
    }
    
    public void finalize() {
        instanceFlag = false;
    }
}