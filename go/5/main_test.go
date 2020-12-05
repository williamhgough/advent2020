package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_parseSeatAllocation(t *testing.T) {
	type args struct {
		instruction string
	}
	tests := []struct {
		name         string
		args         args
		expectedSeat int
	}{
		{
			"Parse FBFBBFFRLR correctly",
			args{
				instruction: "FBFBBFFRLR",
			},
			357,
		},
		{
			"Parse BFFFBBFRRR correctly",
			args{
				instruction: "BFFFBBFRRR",
			},
			567,
		},
		{
			"Parse FFFBBBFRRR correctly",
			args{
				instruction: "FFFBBBFRRR",
			},
			119,
		},
		{
			"Parse BBFFBBFRLL correctly",
			args{
				instruction: "BBFFBBFRLL",
			},
			820,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			res := parseSeatAllocation(tt.args.instruction)

			assert.Equal(t, tt.expectedSeat, res)
		})
	}
}
