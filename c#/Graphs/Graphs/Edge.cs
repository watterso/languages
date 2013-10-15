using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Graphs
{
    public class Edge<T>
    {
        public enum Direction { AtoB = 1, BtoA = -1, None = 0 }
        private Node<T> _NodeA;
        public Node<T> NodeA
        {
            get
            {
                return _NodeA;
            }
            set
            {
                if (_NodeA != null)
                {
                    _NodeA.RemoveEdge(this);
                }
                _NodeA = value;
                if (_NodeA != null)
                {
                    _NodeA.AddEdge(this);
                }

            }
        }
        private Node<T> _NodeB;
        public Node<T> NodeB
        {
            get
            {
                return _NodeB;
            }
            set
            {
                if (_NodeB != null)
                {
                    _NodeB.RemoveEdge(this);
                }
                _NodeB = value;
                if (_NodeB != null)
                {
                    _NodeB.AddEdge(this);
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
