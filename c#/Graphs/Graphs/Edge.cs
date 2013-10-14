using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Graphs
{
    public class Edge<T>
    {
        public enum Direction { AtoB = 1, BtoA = -1, None = 0 }
        public Node<T> NodeA
        {
            get
            {
                return NodeA;
            }
            set
            {
                if (NodeA != null)
                {
                    NodeA.RemoveEdge(this);
                }
                NodeA = value;
                if (NodeA != null)
                {
                    NodeA.AddEdge(this);
                }

            }
        }
        public Node<T> NodeB
        {
            get
            {
                return NodeB;
            }
            set
            {
                if (NodeB != null)
                {
                    NodeB.RemoveEdge(this);
                }
                NodeB = value;
                if (NodeB != null)
                {
                    NodeB.AddEdge(this);
                }

            }
        }
        public Direction EdgeDirection { get; set; }
        public Int32 Length { get; set; }
        public Boolean Visited { get; set; }

        public Edge(Node<T> a, Node<T> b, Direction dir = Direction.None, Int32 len = 0)
        {
            NodeA = a;
            NodeB = b;
            EdgeDirection = dir;
            Length = len;
            Visited = false;
        }
        /// <summary>
        /// Returns whether it is possible to cross edge starting from startNode
        /// </summary>
        /// <param name="startNode"></param>
        /// <returns></returns>
        public Boolean CanCross(Node<T> startNode)
        {
            return startNode == null ? false :
                (startNode == NodeA && EdgeDirection == Direction.AtoB || EdgeDirection == Direction.None) ||
                (startNode == NodeB && EdgeDirection == Direction.BtoA || EdgeDirection == Direction.None);
        }
        /// <summary>
        /// Removes Edge from edge list of both nodes
        /// </summary>
        public void Unlink()
        {
            NodeA = null;
            NodeB = null;
        }
    }
}
