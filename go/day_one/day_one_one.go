package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

const year = 2020

func main() {
	f, err := os.Open("../input/1.txt")
	if err != nil {
		fmt.Println(err)
	}

	seen := map[int]int{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		num, err := strconv.Atoi(scanner.Text())
		if err != nil {
			fmt.Println(err)
		}

		// first star
		if val, ok := seen[year-num]; ok {
			fmt.Println(num * val)
		}

		// second star
		for _, num2 := range seen {
			if val, ok := seen[year-(num+num2)]; ok {
				fmt.Println(val * num * num2)
			}
		}
		seen[num] = num
	}

	if err := scanner.Err(); err != nil {
		fmt.Println(err)
	}
}
