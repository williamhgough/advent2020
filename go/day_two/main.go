package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	var err error
	// passwords must match this pattern.
	re := regexp.MustCompile(`([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)`)

	f, err := os.Open("../../input/2.txt")
	if err != nil {
		fmt.Println(err)
	}

	validPassCount := 0
	count := 0
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		// parse out the password policy
		for _, line := range re.FindAllStringSubmatch(scanner.Text(), -1) {
			matches := line[1:]
			count++

			min, _ := strconv.Atoi(matches[0])
			max, _ := strconv.Atoi(matches[1])
			char := matches[2]
			password := matches[3]

			// get the count of how many times that letter occurs in the password
			// count := strings.Count(password, char)
			// has to be greater/equal to min or lesser/equal to max to be valid
			// if count >= min && count <= max {
			// 	validPassCount++
			// }

			if string(password[min-1]) != char && string(password[max-1]) == char {
				validPassCount++
			}

			if string(password[min-1]) == char && string(password[max-1]) != char {
				validPassCount++
			}
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Println(err)
	}

	fmt.Println(validPassCount)
}
