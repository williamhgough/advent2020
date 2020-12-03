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

	fmt.Println(solveSlope(grid, 3, 1))
	fmt.Println(
		solveSlope(grid, 1, 1) *
			solveSlope(grid, 3, 1) *
			solveSlope(grid, 5, 1) *
			solveSlope(grid, 7, 1) *
			solveSlope(grid, 1, 2),
	)
}

func solveSlope(slope []string, right int, down int) int {
	x, y, trees := 0, 0, 0
	for y < len(slope)-1 {
		x += right
		y += down
		line := slope[y]
		currentX := x % len(line)

		if string(line[currentX]) == "#" {
			trees++
		}
	}
	return trees
}
