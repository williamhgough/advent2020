package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"regexp"
	"strconv"
	"strings"
)

type Passport struct {
	cid string
	pid string
	hcl string
	hgt string
	ecl string
	eyr string
	byr string
	iyr string
}

func main() {
	data, err := ioutil.ReadFile("../../input/4.txt")
	if err != nil {
		log.Fatalf(err.Error())
	}

	valid := 0
	lines := strings.Split(string(data), "\n\n")
	for _, line := range lines {
		passport := parsePassportData(strings.Fields(line))
		if ok := passport.isValid(); ok {
			valid++
		}
	}

	fmt.Println(valid)
}

func parsePassportData(lines []string) *Passport {
	passport := &Passport{}
	for _, match := range lines {
		switch {
		case strings.Contains(match, "cid"):
			passport.cid = strings.ReplaceAll(match, "cid:", "")
		case strings.Contains(match, "pid"):
			passport.pid = strings.ReplaceAll(match, "pid:", "")
		case strings.Contains(match, "hcl"):
			passport.hcl = strings.ReplaceAll(match, "hcl:", "")
		case strings.Contains(match, "hgt"):
			passport.hgt = strings.ReplaceAll(match, "hgt:", "")
		case strings.Contains(match, "ecl"):
			passport.ecl = strings.ReplaceAll(match, "ecl:", "")
		case strings.Contains(match, "eyr"):
			passport.eyr = strings.ReplaceAll(match, "eyr:", "")
		case strings.Contains(match, "byr"):
			passport.byr = strings.ReplaceAll(match, "byr:", "")
		case strings.Contains(match, "iyr"):
			passport.iyr = strings.ReplaceAll(match, "iyr:", "")
		}
	}
	return passport
}

func (p *Passport) isValid() bool {
	// PID
	pid, err := strconv.Atoi(p.pid)
	if err != nil {
		return false
	}
	if p.pid == "" || len(p.pid) != 9 || pid < 0 {
		return false
	}

	// HCL
	if p.hcl == "" || len(p.hcl) != 7 || regexp.MustCompile(`(?m)#([0-9a-z]){6}`).FindString(p.hcl) == "" {
		return false
	}

	// HGT
	if p.hgt == "" || len(p.hgt) > 5 {
		return false
	} else {
		switch {
		case strings.Contains(p.hgt, "in"):
			x := strings.Replace(p.hgt, "in", "", 1)
			xInt, err := strconv.Atoi(x)
			if err != nil {
				return false
			}

			if xInt < 59 || xInt > 76 {
				return false
			}
		case strings.Contains(p.hgt, "cm"):
			x := strings.Replace(p.hgt, "cm", "", 1)
			xInt, err := strconv.Atoi(x)
			if err != nil {
				return false
			}

			if xInt < 150 || xInt > 193 {
				return false
			}
		default:
			return false
		}
	}

	// ECL
	switch p.ecl {
	case "amb", "blu", "brn", "gry", "grn", "hzl", "oth":
	default:
		return false
	}

	// EYR
	eyr, err := strconv.Atoi(p.eyr)
	if err != nil {
		return false
	}
	if p.eyr == "" || len(p.eyr) != 4 || eyr < 2020 || eyr > 2030 {
		return false
	}

	// BYR
	byr, err := strconv.Atoi(p.byr)
	if err != nil {
		return false
	}
	if p.byr == "" || len(p.byr) != 4 || byr < 1920 || byr > 2002 {
		return false
	}

	// IYR
	iyr, err := strconv.Atoi(p.iyr)
	if err != nil {
		return false
	}
	if p.iyr == "" || len(p.iyr) != 4 || iyr < 2010 || iyr > 2020 {
		return false
	}

	return true
}
