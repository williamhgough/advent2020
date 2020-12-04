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
	lines := splitByEmptyNewline(string(data))
	for _, line := range lines {
		passport := parsePassportData(strings.Fields(line))
		if ok := passport.isValid(); ok {
			valid++
		}
	}

	fmt.Println(valid)
}

func splitByEmptyNewline(str string) []string {
	strNormalized := regexp.
		MustCompile("\r\n").
		ReplaceAllString(str, "\n")

	return regexp.
		MustCompile(`\n\s*\n`).
		Split(strNormalized, -1)
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
	if p.pid == "" {
		return false
	} else {
		xInt, err := strconv.Atoi(p.pid)
		if err != nil {
			return false
		}
		if len(p.pid) != 9 || xInt < 0 {
			return false
		}
	}

	if p.hcl == "" {
		return false
	} else {
		if len(p.hcl) != 7 || regexp.MustCompile(`(?m)#([0-9a-z]){6}`).FindString(p.hcl) == "" {
			return false
		}
	}

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

	if p.ecl == "" {
		return false
	} else {
		switch p.ecl {
		case "amb", "blu", "brn", "gry", "grn", "hzl", "oth":
		default:
			return false
		}
	}

	if p.eyr == "" {
		return false
	} else {
		xInt, err := strconv.Atoi(p.eyr)
		if err != nil {
			return false
		}
		if len(p.eyr) != 4 || xInt < 2020 || xInt > 2030 {
			return false
		}
	}

	if p.byr == "" {
		return false
	} else {
		xInt, err := strconv.Atoi(p.byr)
		if err != nil {
			return false
		}

		if len(p.byr) != 4 || xInt < 1920 || xInt > 2002 {
			return false
		}
	}

	if p.iyr == "" {
		return false
	} else {
		xInt, err := strconv.Atoi(p.iyr)
		if err != nil {
			return false
		}
		if len(p.iyr) != 4 || xInt < 2010 || xInt > 2020 {
			return false
		}
	}
	fmt.Printf("%+v\n", p)

	return true
}
