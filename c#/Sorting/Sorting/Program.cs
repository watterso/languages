using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Sorting
{
    public class Program
    {
        public static void Main(string[] args)
        {
            List<int> test = new List<int>();
            test.Add(6);
            test.Add(5);
            test.Add(3);
            test.Add(1);
            test.Add(8);
            test.Add(7);
            test.Add(2);
            test.Add(4);
            List<int> oot = IntMergeSort(test);
            StringBuilder str = new StringBuilder();
            foreach (int a in oot)
            {
                str.Append(" " + a);
            }
            System.Diagnostics.Debug.WriteLine("------L'Output------");
            System.Diagnostics.Debug.WriteLine(str.ToString());
        }
        public static List<int> IntMergeSort(List<int> array)
        {
            if (array.Count <= 1)
                return array;
            List<int> first = new List<int>();
            for (int i = 0; i < array.Count / 2; i++)
            {
                first.Add(array.ElementAt(i));
            }
            first = IntMergeSort(first);
            List<int> second = new List<int>();
            for (int j = array.Count / 2; j < array.Count; j++)
            {
                second.Add(array.ElementAt(j));
            }
            second = IntMergeSort(second);
            int maxLength = first.Count >= second.Count ? first.Count : second.Count;
            List<int> oot = new List<int>();
            while (first.Count > 0 && second.Count > 0)
            {
                int a = first.ElementAt(0);
                int b = second.ElementAt(0);
                if (a <= b)
                {
                    oot.Add(a);
                    first.RemoveAt(0);
                }
                else
                {
                    oot.Add(b);
                    second.RemoveAt(0);
                }
            }
            if (first.Count > 0)
            {
                oot.AddRange(first);
            }
            if (second.Count > 0)
            {
                oot.AddRange(second);
            }
            return oot;
        }
        public static List<int> IntQuickSort(List<int> array)
        {
            if (array.Count <= 1)
                return array;
            int pivot = array.ElementAt(array.Count / 2);
            array.RemoveAt(array.Count / 2);
            List<int> less = new List<int>();
            List<int> greater = new List<int>();
            foreach (int a in array)
            {
                if (a <= pivot)
                {
                    less.Add(a);
                }
                else
                {
                    greater.Add(a);
                }
            }
            List<int> pivList = new List<int>();
            pivList.Add(pivot);
            return IntQuickSort(less).Concat(pivList).ToList<int>().Concat(IntQuickSort(greater)).ToList<int>();
        }
    }
}
