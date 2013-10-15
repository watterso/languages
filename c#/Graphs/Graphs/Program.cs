using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Graphs
{
    public class Program
    {
        public static void Main(string[] args)
        {
            //TODO: Adapt this to load graph
            string[] lines = System.IO.File.ReadAllLines(@"StateAdjacency.txt");
            Graph<string> graph = new Graph<string>();
            Console.WriteLine("HERE 0.");
            foreach (string line in lines)
            {
                string[] nodes = line.Split(' ');
                //Console.WriteLine(nodes[0]+" "+nodes[1]);
                Node<string> a = graph.GetNode(nodes[0]);
                Node<string> b = graph.GetNode(nodes[1]);
                if (a==null)
                {
                    a = new Node<string>(nodes[0]);
                    graph.AddNode(a);
                }
                if (b == null)
                {
                    b = new Node<string>(nodes[1]);
                    graph.AddNode(b);
                }
                if (!a.Connected(b))
                {
                    Edge<string> e = new Edge<string>(a,b);
                    graph.AddEdge(e);
                }
            }
            Console.WriteLine(graph.PrintNodes());
            // Keep the console window open in debug mode.
            Console.WriteLine("Press any key to exit.");
            System.Console.ReadKey();
        }
    }
}
