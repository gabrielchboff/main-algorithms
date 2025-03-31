namespace BinaryTreeCSharp;

public class BinaryTree : IBinaryTree
{
    private Node? Root { get; set; }
    private int _keycounter = 0;

    public void Clear()
    {
        Root = null;
    }

    public bool IsEmpty()
    {
        return Root == null;
    }

    public Node? Search(int key)
    {
        Node? currentNode = Root;

        while (currentNode != null)
        {
            if (currentNode.Key == key)
            {
                return currentNode;
            }
            else if (key < currentNode.Key)
            {
                currentNode = currentNode.Left;
            }
            else
            {
                currentNode = currentNode.Right;
            }
        }

        return currentNode;
    }

    public void Insert(string value, Node? currentNode = null)
    {
        if (Root == null)
        {
            Root = new Node(_keycounter++, value, null!, null!);
            return;
        }

        if (Root.Value.Length > value.Length)
        {
            if (Root.Left == null!)  
            {
                Root.Left = new Node(_keycounter++, value, null!, null!);
                return;
            }
            Insert(value, Root.Left);
            
        }
        else if (Root.Value.Length < value.Length)
        {
            if (Root.Right == null!)
            {
                Root.Right = new Node(_keycounter++, value, null!, null!);
                return;
            }

            Insert(value, Root.Right);
        }
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