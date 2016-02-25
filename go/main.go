package main

import (
	"gopkg.in/redis.v3"
	"io"
	"log"
	"net/http"
)

func main() {
	client := redis.NewClient(&redis.Options{
		Addr: "localhost:6379",
	})

	http.HandleFunc("/get", func(w http.ResponseWriter, r *http.Request) {
		key := r.FormValue("key")

		if key == "" {
			io.WriteString(w, "missing param 'key'")
			w.WriteHeader(http.StatusBadRequest)
			return
		}

		val, err := client.Get(key).Result()

		if err == redis.Nil {
			w.WriteHeader(http.StatusNotFound)
			io.WriteString(w, "not found")
		} else if err != nil {
			w.WriteHeader(http.StatusInternalServerError)
			io.WriteString(w, "operation error")
			return
		} else {
			io.WriteString(w, val)
		}
	})

	log.Fatal(http.ListenAndServe(":8000", nil))
}
