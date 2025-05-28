/*
 * To change this template, choose Tools | Templates
 * and open the template in the editor.
 */
package FactoryMethod.exemplo;

/**
 *
 * @author kleinnerfarias
 */
public abstract class Document {
    public abstract void open();
    public abstract void close();
    public abstract void save();
    public abstract void revert();
}
