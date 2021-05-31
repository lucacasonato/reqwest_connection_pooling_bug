package main

import (
	"fmt"
	"net/http"
)

func main() {
	done := make(chan int)

	client := http.Client{}

	for i := 0; i < 100; i++ {
		go loop(i, client, done)
	}
	for i := 0; i < 100; i++ {
		<-done
	}
}

func loop(i int, client http.Client, done chan int) {
	for i := 0; i < 20; i++ {
		res, err := client.Get("https://deno-website2.now.sh/")
		if err != nil {
			panic(err)
		}

		if res.StatusCode != 200 {
			panic("not 200!")
		}
	}
	fmt.Printf("task %2v done\n", i)
	done <- 0
}
