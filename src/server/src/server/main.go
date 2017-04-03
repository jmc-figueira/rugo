package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

type match struct {
	ip string
}

func main() {
	http.HandleFunc("/matches", getMatches)

	log.Fatal(http.ListenAndServe(":8080", nil))
}

func getMatches(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "test")
}
