/*
 * To change this template, choose Tools | Templates
 * and open the template in the editor.
 */
package FactoryMethod.exemplo;

/**
 *
 * @author kleinnerfarias
 */
public class Client {
    
    public static void main(String arg[]){
    
        MyApplication myappA = new MyApplication();
        Document docA = myappA.createDocument("MyDocument");
        docA.open();
        
        MyApplication myappB= new MyApplication();
        Document docB = myappB.createDocumentAux("MyDocument");
        docB.open();
      
    }
    
}
