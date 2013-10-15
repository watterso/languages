using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Graphs
{
    public class Graph<T>
    {
        public List<Node<T>> Nodes { get; private set; }
        public List<Edge<T>> Edges { get; private set; }
        public Graph()
        {
            Nodes = new List<Node<T>>();
            Edges = new List<Edge<T>>();
        }
        public void AddNode(Node<T> node)
        {
            Nodes.Add(node);
        }
        public void RemoveNode(Node<T> node)
        {
            Nodes.Remove(node);
            if (node != null)
            {
                foreach (Edge<T> edge in node.ConnectedEdges)
                {
                    edge.Unlink();
                    Edges.Remove(edge);
                }
            }
        }
        public Boolean Contains(Node<T> a)
        {
            return Nodes.Contains(a);
        }
        public Boolean Contains(T d)
        {
            return GetNode(d) != null;
        }
        /// <summary>
        /// Returns the first Node with contents equal to val
        /// </summary>
        /// <param name="val">Search term</param>
        /// <returns></returns>
        public Node<T> GetNode(T val)
        {
            foreach (Node<T> node in Nodes)
            {
                if (node.Content.Equals(val))
                {
                    return node;
                }
            }
            return null;
        }
        /// <summary>
        /// Contructs an edge and adds it (and the nodes if necessary) to the Graph
        /// </summary>
        /// <param name="a">Node A</param>
        /// <param name="b">Node B</param>
        /// <param name="dir">Direction (optional)</param>
        /// <param name="len">Length (optional)</param>
        public void AddEdge(Node<T> a, Node<T> b, Edge<T>.Direction dir = Edge<T>.Direction.None, Int32 len = 0)
        {
            if (!Nodes.Contains(a))
            {
                AddNode(a);
            }
            if (!Nodes.Contains(b))
            {
                AddNode(b);
            }
            Edge<T> edge = new Edge<T>(a, b, dir, len);
            Edges.Add(edge);
        }
        public void AddEdge(Edge<T> e)
        {
            Edges.Add(e);
        }
        public string PrintNodes()
        {
            string ret = "";
            foreach (Node<T> n in Nodes)
            {
                ret += n.Content.ToString() + ",";
            }
            return ret;
        }
    }
}
