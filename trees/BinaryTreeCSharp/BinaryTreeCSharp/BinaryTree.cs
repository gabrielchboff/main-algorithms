namespace BinaryTreeCSharp;

public class BinaryTree : IBinaryTree
{
    private Node? Root { get; set; }

    public void Clear()
    {
        Root = null;
    }

    public bool IsEmpty()
    {
        return Root == null;
    }

    public Node Search(int key)
    {
        throw new NotImplementedException();
    }

    public void Insert(int key, string value)
    {
        throw new NotImplementedException();
    }

    public void Delete(int key)
    {
        throw new NotImplementedException();
    }

    public void PreOrderTraversal()
    {
        throw new NotImplementedException();
    }

    public void InOrderTraversal()
    {
        throw new NotImplementedException();
    }

    public void PostOrderTraversal()
    {
        throw new NotImplementedException();
    }

    public void LevelOrderTraversal()
    {
        throw new NotImplementedException();
    }
}