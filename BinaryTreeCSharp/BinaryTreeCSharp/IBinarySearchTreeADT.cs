namespace BinaryTreeCSharp;

public interface IBinarySearchTreeADT
{
    public void Clear();
    public bool IsEmpty();
    public Node? Search(int key);
    public void Insert(string value, Node? currentNode = null);
    public void Delete(int key);
    public void PreOrderTraversal();
    public void InOrderTraversal();
    public void PostOrderTraversal();
    public void LevelOrderTraversal();
    public int CountInternal();
    public int Degrees(int key);
    public int Height(int key);
    public int Level(int key);
    public string Ancestor(int key);
}