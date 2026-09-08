package main

import "testing"

func TestFib(t *testing.T) {
	if got := Fib(20); got != 6765 {
		t.Fatalf("got %d", got)
	}
}
