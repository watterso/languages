package main

import "fmt"

func main() {
	data := []int{6, 5, 3, 1, 8, 7, 2, 4}
	data1 := make([]int, len(data))
	copy(data1, data)
	fmt.Println(data)
	fmt.Println(IntQuickSort(data, 0))
	fmt.Println(data1)
	fmt.Println(IntQuickSort(data1, 1))

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
