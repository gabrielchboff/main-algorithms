namespace BinaryTreeCSharp;

public interface IBinaryTree
{
    public void Clear();
    public bool IsEmpty();
    public Node Search(int key);
    public void Insert(int key, string value);
    public void Delete(int key);
    public void PreOrderTraversal();
    public void InOrderTraversal();
    public void PostOrderTraversal();
    public void LevelOrderTraversal();
}