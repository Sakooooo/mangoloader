package main

import (
	"embed"
	"fmt"
	"html/template"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

//go:embed templates
var templateFS embed.FS

//go:embed static
var staticFS embed.FS

func main() {
	fmt.Println("Hello")

	r := chi.NewRouter()
	r.Use(middleware.Logger)

	tmpl, err := template.ParseGlob("templates/*")
	if err != nil {
		fmt.Println("failed to load templates: ", err)
		return
	}

	// staticServ, err := fs.Sub(staticFS, "")
	// if err != nil {
	// 	fmt.Println("failed to prepare static dir: ", err)
	// 	return
	// }

	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		// w.Write([]byte("hello"))

		err = tmpl.ExecuteTemplate(w, "index.html", nil)
		if err != nil {
			http.Error(w, "Failed to render template.", http.StatusInternalServerError)
			fmt.Println("Failed to render template: ", err)
			return
		}

	})

	r.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.FS(staticFS))))

	test, err := staticFS.ReadDir(".")
	if err != nil {
		fmt.Println(err)
		return
	}

	for _, f := range test {
		fmt.Println(f.Name())
	}

	fmt.Println()

	fmt.Println("Listening on :3000")

	http.ListenAndServe(":3000", r)
}
