package main

import (
	"embed"
	"fmt"
	sq "github.com/Masterminds/squirrel"
	"html/template"
	"net/http"
	_ "os"

	"github.com/Sakooooo/mangoloader/internal/config"
	"github.com/Sakooooo/mangoloader/internal/database"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

//go:embed templates
var templateFS embed.FS

//go:embed static
var staticFS embed.FS

type indexData struct {
}

func main() {
	fmt.Println("Hello")

	config, err := config.ReadConfig("config.example.toml")
	if err != nil {
		fmt.Println(err)
		return
	}

	targetHost := config.Server.Host + ":" + config.Server.Port

	db, err := database.InitDB(config)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println(db)

	r := chi.NewRouter()
	r.Use(middleware.Logger)

	tmpl, err := template.ParseGlob("templates/*")
	// tmpl, err := template.ParseFS(templateFS, "templates/*.html")
	if err != nil {
		fmt.Println("failed to load templates: ", err)
		return
	}

	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		// w.Write([]byte("hello"))

		mangaQuery := sq.Select("title", "source").From("manga")
		manga, err := mangaQuery.RunWith(db.DB).Query()
		if err != nil {
			http.Error(w, "Failed to query database.", http.StatusInternalServerError)
			fmt.Println("Failed to query DB: ", err)
		}

		// need to query
		// see if can use struct

		err = tmpl.ExecuteTemplate(w, "index.html", nil)
		if err != nil {
			http.Error(w, "Failed to render template.", http.StatusInternalServerError)
			fmt.Println("Failed to render template: ", err)
			return
		}

	})

	// r.Handle("/static/*", http.FileServer(http.FS(staticFS)))
	r.Handle("/static/*", http.StripPrefix("/static/", http.FileServer(http.Dir("static"))))

	fmt.Println("Listening on ", targetHost)

	http.ListenAndServe(targetHost, r)
}
