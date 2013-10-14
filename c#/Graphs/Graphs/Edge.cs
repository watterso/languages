using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Graphs
{
    public class Edge<T>
    {
        public enum Direction { AtoB = 1, BtoA = -1, None = 0 }
        public Node<T> nodeA
        {
            get
            {
                return nodeA;
            }
            set
            {
                if (nodeA != null)
                {
                    nodeA.RemoveEdge(this);
                }
                nodeA = value;
                if (nodeA != null)
                {
                    nodeA.AddEdge(this);
                }

            }
        }
        public Node<T> nodeB
        {
            get
            {
                return nodeB;
            }
            set
            {
                if (nodeB != null)
                {
                    nodeB.RemoveEdge(this);
                }
                nodeB = value;
                if (nodeB != null)
                {
                    nodeB.AddEdge(this);
                }

            }
        }
        public Direction direction { get; set; }
        public Int32 length { get; set; }
        public Boolean visited { get; set; }

        public Edge(Node<T> a, Node<T> b, Direction dir = Direction.None, Int32 len = 0)
        {
            nodeA = a;
            nodeB = b;
            direction = dir;
            length = len;
            visited = false;
        }
        /// <summary>
        /// Returns whether it is possible to cross edge starting from startNode
        /// </summary>
        /// <param name="startNode"></param>
        /// <returns></returns>
        public Boolean CanCross(Node<T> startNode)
        {
            return startNode == null ? false :
                (startNode == nodeA && direction == Direction.AtoB || direction == Direction.None) ||
                (startNode == nodeB && direction == Direction.BtoA || direction == Direction.None);
        }
        /// <summary>
        /// Removes Edge from edge list of both nodes
        /// </summary>
        public void Unlink()
        {
            nodeA = null;
            nodeB = null;
        }
    }
}
