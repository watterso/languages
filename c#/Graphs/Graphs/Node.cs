using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Graphs
{
    public class Node<T>
    {
        public List<Edge<T>> connectedEdges { get; private set; }
        public T content { get; set; }

        public Node (T data)
        {
            content = data;
            connectedEdges = new List<Edge<T>>();
        }

        public void RemoveEdge(Edge<T> edge)
        {
            connectedEdges.Remove(edge);
        }
        public void AddEdge(Edge<T> edge)
        {
            connectedEdges.Add(edge);
        }
    }
}
