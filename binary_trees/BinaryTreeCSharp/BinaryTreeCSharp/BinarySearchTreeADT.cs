using System.Collections.Generic;

namespace BinaryTreeCSharp;


public class BinarySearchTreeADT
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

    //TODO: Consider implementing this first
    
    public int CountInternal()
    {
        int count = 0;
        if (Root == null) return count;

        Queue<Node> queue = new Queue<Node>();
        queue.Enqueue(Root);

        while (queue.Count > 0)
        {
            Node currentNode = queue.Dequeue();

            if (currentNode.Left != null)
            {
                queue.Enqueue(currentNode.Left);
            }

            if (currentNode.Right != null)
            {
                queue.Enqueue(currentNode.Right);
            }
        }

        count = queue.Count;
        return count;
    }
    
    public int Degree(int key)
    {
        if(Root == null) return -1;
        
        int degree = 0;
        Node? currentNode = Search(key);
        
        if (currentNode == null) return -1;
        if (currentNode.Left != null) degree++;
        if (currentNode.Right != null) degree++;
        
        return degree;
        
    }
    
    public int Height(int key)
    {
        if (Root == null) return -1;
        
        int height = 0;
        Node? currentNode = Search(key);
        
        if (currentNode == null) return -1;
        
        if (currentNode.Left != null)
        {
            int leftHeight = 1 + Height(currentNode.Left.Key);
            if (leftHeight > height) height = leftHeight;
        }
        
        if (currentNode.Right != null)
        {
            int rightHeight = 1 + Height(currentNode.Right.Key);
            if (rightHeight > height) height = rightHeight;
        }
        
        return height;
    }
    
    public int Level(int key)
    {
        if (Root == null) return -1;
        
        int level = 0;
        
        
        return level;
      
    }

    public string Ancestor(int key)
    { 
        throw new NotImplementedException();
    }
}