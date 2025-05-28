/*
 * To change this template, choose Tools | Templates
 * and open the template in the editor.
 */
package FactoryMethod.exemplo;

/**
 *
 * @author kleinnerfarias
 */
public class MyDocument extends Document{
   
    @Override
    public void open() {
        System.out.print("Method open was performed.");
    }

    @Override
    public void close() {
        System.out.print("Method close was performed.");
    }

    @Override
    public void save() {
        System.out.print("Method save was performed.");
    }

    @Override
    public void revert() {
        System.out.print("Method revert was performed.");
    }

}
