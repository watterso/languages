using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Graphs
{
    public class Node<T>
    {
        public List<Edge<T>> ConnectedEdges { get; private set; }
        public T Content { get; set; }

        public Node (T data)
        {
            Content = data;
            ConnectedEdges = new List<Edge<T>>();
        }

        public void RemoveEdge(Edge<T> edge)
        {
            ConnectedEdges.Remove(edge);
        }
        public void AddEdge(Edge<T> edge)
        {
            ConnectedEdges.Add(edge);
        }
        public Boolean Connected(Node<T> n)
        {
            foreach (Edge<T> e in ConnectedEdges)
            {
                if (e.NodeA.Equals(this))
                {
                    if (e.NodeB.Equals(n) && e.CanCross(n))
                    {
                        return true;
                    }
                }
                else if (e.NodeB.Equals(this))
                {
                    if (e.NodeA.Equals(n) && e.CanCross(n))
                    {
                        return true;
                    }
                }
            }
            return false;
        }
    }
}
