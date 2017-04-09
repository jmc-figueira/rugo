package main

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/gorilla/mux"
	"google.golang.org/appengine"
	"google.golang.org/appengine/datastore"
)

type Match struct {
	ip string
	name string
	max_slots uint64
	current_slots uint64
}

func (m *Match) getIP() string{
	return m.ip
}

func main() {
	router := mux.NewRouter()

	router.HandleFunc("/matches", getMatches).Methods("GET")
}

func getMatches(w http.ResponseWriter, r *http.Request) {

	ctx := appengine.NewContext(r)

	datastore.Get(ctx, datastore.NewKey(), new(Match))

	fmt.Fprintln(w, "test")
}
