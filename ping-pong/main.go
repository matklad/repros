package main

import (
	"fmt"
	"time"
	"sync"
)

const N = 1_000_000;

func main() {
	t := time.Now()
	aToB := make(chan uint64)
	bToA := make(chan uint64)

	var wg sync.WaitGroup
	wg.Add(2)
	go worker(&wg, true, bToA, aToB)
	go worker(&wg, false, aToB, bToA)
	wg.Wait()

	fmt.Println(time.Since(t))
}

func worker(wg *sync.WaitGroup, init bool, receiver <-chan uint64, sender chan<- uint64) {
	if init {
		sender <- 0
	}
	for x := range receiver {
		if x == N {
			break
		}
		sender <- x + 1
	}
	close(sender)
	wg.Done()
}
