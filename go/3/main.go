package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	f, err := os.Open("../../input/3.txt")
	if err != nil {
		fmt.Println(err)
	}

	grid := []string{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		row := scanner.Text()
		grid = append(grid, row)
	}

	if err := scanner.Err(); err != nil {
		fmt.Println(err)
	}

	target := "#"
	fmt.Println(calculateGridHits(grid, target, 3, 1))
	fmt.Println(
		calculateGridHits(grid, target, 1, 1) *
			calculateGridHits(grid, target, 3, 1) *
			calculateGridHits(grid, target, 5, 1) *
			calculateGridHits(grid, target, 7, 1) *
			calculateGridHits(grid, target, 1, 2),
	)
}

func calculateGridHits(grid []string, target string, startingX int, startingY int) int {
	x, y, hits := 0, 0, 0
	for y < len(grid)-1 {
		x += startingX
		y += startingY
		line := grid[y]
		currentX := x % len(line)

		if string(line[currentX]) == target {
			hits++
		}
	}
	return hits
}
