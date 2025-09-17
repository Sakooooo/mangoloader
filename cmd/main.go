package main

import (
	"fmt"
	"html/template"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

func main() {
	fmt.Println("Hello")

	r := chi.NewRouter()
	r.Use(middleware.Logger)

	tmpl, err := template.ParseGlob("templates/*")
	if err != nil {
		fmt.Println("failed to load templates: ", err)
		return
	}

	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		// w.Write([]byte("hello"))

		err = tmpl.ExecuteTemplate(w, "index.html", nil)
		if err != nil {
			http.Error(w, "Failed to render template.", http.StatusInternalServerError)
			fmt.Println("Failed to render template: ", err)
			return
		}

	})

	fmt.Println("Listening on :3000")

	http.ListenAndServe(":3000", r)
}
