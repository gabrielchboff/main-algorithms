/*
 * To change this template, choose Tools | Templates
 * and open the template in the editor.
 */
package FactoryMethod.exemplo;

import java.util.HashMap;

/**
 *
 * @author kleinnerfarias
 */
public class MyApplication extends Application {

    private HashMap myApplicationMap;
    
    public MyApplication(){
        myApplicationMap = new HashMap();
        registerDocument();
    }
            
    
    @Override
    public Document createDocument(String str){
        /* do lots of things */
        if(str.equalsIgnoreCase("MyDocument")){
             return new MyDocument();
        } else {
          /* ... */
        }             
        return null;        
    }
    
    public Document createDocumentAux(String str){
       return (Document)myApplicationMap.get(str);        
    }
    
  
    private void registerDocument(){
        myApplicationMap.put("MyDocument", new MyDocument());       
    }
    
    private void registerDocument(String str, Document doc){
        myApplicationMap.put(str, doc);       
    }
    
    
}
