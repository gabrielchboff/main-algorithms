namespace BinaryTreeCSharp;

public class Node
{
    private int _key { get; set; }
    private string _value { get; set; }
    private Node? _left { get; set; }
    private Node? _right { get; set; }

    public Node(int key, string value, Node? left, Node? right)
    {
        _key = key;
        _value = value;
        _left = left;
        _right = right;
    }
    
    public int Key { get => _key; set => _key = value; }
    public string Value { get => _value; set => _value = value; }
    public Node? Left { get => _left; set => _left = value; }
    public Node? Right { get => _right; set => _right = value; }
    
    public override string ToString() => $"Key: {_key}, Value: {_value}";
    
    public Node? Next(Node otherKey) => _key < otherKey.Key ? _right : _left;
    
    
}