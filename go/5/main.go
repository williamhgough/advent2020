package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("../../input/5.txt")
	if err != nil {
		fmt.Println(err)
	}

	ids := []int{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		seatID := parseSeatAllocation(scanner.Text())
		ids = append(ids, seatID)
	}

	if err := scanner.Err(); err != nil {
		fmt.Println(err)
	}

	// Part 1
	// element at last index is the highest Seat ID.
	sort.Ints(ids)
	fmt.Println(ids[len(ids)-1])

	// Part 2
	for i := 0; i < len(ids)-1; i++ {
		// if the current index + 1 does not equal the next index
		// then it's our ID: "the seats with IDs +1 and -1 from yours will be in your list"
		if ids[i]+1 != ids[i+1] {
			fmt.Println(ids[i] + 1)
		}
	}
}

// Using:
// - https://stackoverflow.com/questions/9271469/go-convert-string-which-represent-binary-number-into-int
// - https://stackoverflow.com/questions/34215080/strings-replacer-how-to-replace-all-substrings-at-once
// We're able to convert the instructions to binary format and convert to int
func parseSeatAllocation(instruction string) int {
	replacer := strings.NewReplacer("F", "0", "L", "0", "B", "1", "R", "1")
	output := replacer.Replace(instruction)

	row, err := strconv.ParseInt(output[:7], 2, 64)
	if err != nil {
		fmt.Println(err)
	}

	col, err := strconv.ParseInt(output[7:], 2, 64)
	if err != nil {
		fmt.Println(err)
	}

	return int(8*row + col)
}
