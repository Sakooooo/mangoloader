package main

import (
	// "database/sql"
	"embed"
	"fmt"
	"html/template"
	"net/http"
	_ "os"

	sq "github.com/Masterminds/squirrel"

	"github.com/Sakooooo/mangoloader/internal/config"
	"github.com/Sakooooo/mangoloader/internal/database"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

//go:embed templates
var templateFS embed.FS

//go:embed static
var staticFS embed.FS

type IndexData struct {
	Manga []database.Manga
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

		queryBuilder := sq.Select(
			"id",
			"title",
			"artist",
			"cover",
			"source",
		).From("manga")

		query, args, err := queryBuilder.ToSql()
		if err != nil {
			http.Error(w, "Failed to build query.", http.StatusInternalServerError)
			fmt.Println("Failed to build query: ", err)
			return
		}

		rows, err := db.DB.Query(query, args...)
		if err != nil {
			http.Error(w, "Failed to query database.", http.StatusInternalServerError)
			fmt.Println("Failed to query database: ", err)
			return
		}
		defer rows.Close()

		var data IndexData

		for rows.Next() {
			var m database.Manga

			if err := rows.Scan(&m.Id, &m.Title, &m.Artist, &m.Cover, &m.Source); err != nil {
				http.Error(w, "Failed to scan rows", http.StatusInternalServerError)
				fmt.Println("Failed to scan rows: ", err)
				return
			}

			data.Manga = append(data.Manga, m)
		}

		if err = rows.Err(); err != nil {
			http.Error(w, "Query errored.", http.StatusInternalServerError)
			fmt.Println("Query errored: ", err)
			return
		}

		fmt.Printf("Data to be passed to template: %+v\n", data)

		fmt.Println(data)
		fmt.Println(data.Manga)

		err = tmpl.ExecuteTemplate(w, "index.html", data)
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
