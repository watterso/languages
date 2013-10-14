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
    }
}
