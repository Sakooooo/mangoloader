package config

import (
	"bufio"
	"fmt"
	"io"
	"os"

	"github.com/pelletier/go-toml/v2"
)

type database struct {
	Engine    string
	Directory string
	// Host      string
	// Port      string
}

type server struct {
	Host string
	Port string
}

type Config struct {
	Database database
	Server   server
}

func ReadConfig(path string) (Config, error) {
	var cfg Config

	ConfigFile, err := os.Open(path)
	if err != nil {
		fmt.Println("Failed to read Config file: ", err)
		return Config{}, err
	}

	fileinfo, err := ConfigFile.Stat()
	if err != nil {
		fmt.Println("Failed to get Configfile info: ", err)
		return Config{}, err
	}

	bs := make([]byte, fileinfo.Size())
	_, err = bufio.NewReader(ConfigFile).Read(bs)
	if err != nil {
		fmt.Println("Failed to read Config file: ", err)
		return Config{}, err
	}

	err = toml.Unmarshal(bs, &cfg)
	if err != nil && err != io.EOF {
		fmt.Println("Failed to decode Config file: ", err)
		return Config{}, err
	}

	return cfg, nil
}
