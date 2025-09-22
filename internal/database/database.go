package database

import (
	"context"
	"database/sql"

	// sq "github.com/Masterminds/squirrel"
	"github.com/Sakooooo/mangoloader/internal/config"
	_ "modernc.org/sqlite"
)

type DB struct {
	DB *sql.DB
}

type Manga struct {
	title  string
	source string
}

type MangaDBRow struct {
	ID int
	Manga
}

func InitDB(cfg config.Config) (DB, error) {
	db, err := sql.Open("sqlite", cfg.Database.Host)
	if err != nil {
		return DB{}, err
	}

	_, err = db.ExecContext(
		context.Background(),
		`
		CREATE TABLE IF NOT EXISTS manga (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			title TEXT NOT NULL,
			source TEXT NOT NULL
		)
		`,
	)

	if err != nil {
		return DB{}, err
	}

	return DB{DB: db}, nil
}
