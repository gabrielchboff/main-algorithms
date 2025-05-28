/*
 * To change this template, choose Tools | Templates
 * and open the template in the editor.
 */
package FactoryMethod.exemplo;

/**
 *
 * @author kleinnerfarias
 */
public abstract class Application {
    
    public abstract Document createDocument(String str);
    
    public Document newDocument(String str){
        Document newdoc = createDocument(str);
        newdoc.open();
        /** do something else **/
        return newdoc;
    }
    
    public void openDocument(Document doc){
        System.out.print("Method openDocumento was performed.");
        /** do lost of operations **/
        doc.open();
    }

}
