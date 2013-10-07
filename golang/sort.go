package main

import "fmt"

func main() {
	data := []int{6, 5, 3, 9, 1, 8, 7, 2, 4}
	fmt.Println(IntSelect(data, 6))
}

func IntSelect(data []int, k int) int {
	if len(data) <= 5 {
		return IntMergeSort(data, 0)[k]
	}
	medians := make([]int, 0)
	i := 0
	for 5*(i+1) < len(data) {
		interest := IntMergeSort(data[i*5:(i+1)*5], 0)
		medians = append(medians, interest[(int)(len(interest)/2)])
		i++
	}
	//fmt.Println(data)
	medians = IntQuickSort(medians, 0)
	//fmt.Println("Median: ", medians, "|", (int)(len(medians)/2))
	median := medians[(int)(len(medians)/2)]
	less := make([]int, 0)
	greater := make([]int, 0)
	for _, val := range data {
		if val <= median {
			less = append(less, val)
		} else {
			greater = append(greater, val)
		}
	}
	//fmt.Println("Less: ", less, "|", len(less), "| k: ", k)
	if k+1 == len(less) {
		return IntMergeSort(less, 0)[k]
	} else if k < len(less) {
		return IntSelect(less, k)
	} else {
		return IntSelect(greater, k-len(less))
	}
}
func IntQuickSort(data []int, flag int) []int {
	if len(data) <= 1 {
		return data
	}
	ind := (int)(len(data) / 2)
	pivot := data[ind]
	//fmt.Println("PIVOT: ", pivot)
	data = append(data[:ind], data[ind+1:]...)
	//fmt.Println(data)
	less := make([]int, 0)
	greater := make([]int, 0)
	for _, val := range data {
		if flag == 0 {
			if val <= pivot {
				less = append(less, val)
			} else {
				greater = append(greater, val)
			}
		} else {
			if val > pivot {
				less = append(less, val)
			} else {
				greater = append(greater, val)
			}
		}
	}
	return append(append(IntQuickSort(less, flag), pivot), IntQuickSort(greater, flag)...)
}
func IntMergeSort(data []int, flag int) []int {
	//fmt.Println(data)
	if len(data) <= 1 {
		return data
	}
	half := (int)(len(data) / 2)
	left := append(data[:half])
	right := append(data[half:])
	left = IntMergeSort(left, flag)
	right = IntMergeSort(right, flag)
	val := 0
	out := make([]int, 0)
	for len(left) > 0 && len(right) > 0 {
		if flag == 0 {
			if left[0] < right[0] {
				val, left = left[0], left[1:]
				out = append(out, val)
			} else {
				val, right = right[0], right[1:]
				out = append(out, val)
			}
		} else {
			if left[0] > right[0] {
				val, left = left[0], left[1:]
				out = append(out, val)
			} else {
				val, right = right[0], right[1:]
				out = append(out, val)
			}
		}
	}
	if len(left) > 0 {
		out = append(out, left...)
	}
	if len(right) > 0 {
		out = append(out, right...)
	}
	return out

}
