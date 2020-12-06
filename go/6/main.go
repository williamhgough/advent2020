package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

func main() {
	data, err := ioutil.ReadFile("../../input/6.txt")
	if err != nil {
		log.Fatalf(err.Error())
	}

	var groupAnswers []map[string]int
	groupSize := map[int]int{}
	groups := strings.Split(string(data), "\n\n")
	for i, group := range groups {
		groupMembers := strings.Split(group, "\n")
		groupSize[i] = len(groupMembers)
		groupAnswers = append(groupAnswers, map[string]int{})
		parseCount(groupAnswers[i], groupMembers)
	}

	count := 0
	for i, group := range groupAnswers {
		for _, answered := range group {
			if answered == groupSize[i] {
				count++
			}
		}
	}

	fmt.Println(count)
}

func parseCount(counts map[string]int, groupMembers []string) {
	for _, member := range groupMembers {
		for _, answer := range member {
			if _, ok := counts[string(answer)]; !ok {
				counts[string(answer)] = 1
			} else {
				counts[string(answer)] += 1
			}
		}
	}
}
