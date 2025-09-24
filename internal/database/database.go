package database

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"os"

	// sq "github.com/Masterminds/squirrel"
	"github.com/Sakooooo/mangoloader/internal/config"
	_ "modernc.org/sqlite"
)

type DB struct {
	DB *sql.DB
}

type Manga struct {
	Id     int
	Title  string
	Artist string
	Cover  string
	Source string
}

func InitDB(cfg config.Config) (DB, error) {

	dbPath := cfg.Database.Directory + "database.db"

	_, err := os.Stat(dbPath)
	if err == nil {
	} else if errors.Is(err, os.ErrNotExist) {
		fmt.Println("Database doesn't exist! Creating...")
		err = os.MkdirAll(cfg.Database.Directory, 0770)
		if err != nil {
			fmt.Println("Failed to create directory ", err)
			return DB{}, err
		}
	} else {
		fmt.Println("can't stat database path, aborting ", err)
		return DB{}, err
	}

	db, err := sql.Open("sqlite", dbPath)
	if err != nil {
		return DB{}, err
	}

	// could we move this somewhere else
	_, err = db.ExecContext(
		context.Background(),
		`
		CREATE TABLE IF NOT EXISTS manga (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			title TEXT NOT NULL,
			artist TEXT NOT NULL,
			cover TEXT NOT NULL,
			source TEXT NOT NULL
		)
		`,
	)

	if err != nil {
		return DB{}, err
	}

	return DB{DB: db}, nil
}
