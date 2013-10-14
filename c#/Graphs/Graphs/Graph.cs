using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Graphs
{
    public class Graph<T>
    {
        public List<Node<T>> nodes { get; private set; }
        public List<Edge<T>> edges { get; private set; }
        public Graph()
        {
            nodes = new List<Node<T>>();
            edges = new List<Edge<T>>();
        }
        public void AddNode(Node<T> node)
        {
            nodes.Add(node);
        }
        public void RemoveNode(Node<T> node)
        {
            nodes.Remove(node);
            if (node != null)
            {
                foreach (Edge<T> edge in node.connectedEdges)
                {
                    edge.Unlink();
                    edges.Remove(edge);
                }
            }
        }
        /// <summary>
        /// Returns the first Node with contents equal to val
        /// </summary>
        /// <param name="val">Search term</param>
        /// <returns></returns>
        public Node<T> getNode(T val)
        {
            foreach (Node<T> node in nodes)
            {
                if (node.content.Equals(val))
                {
                    return node;
                }
            }
            return null;
        }
    }
}
