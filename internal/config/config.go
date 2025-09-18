package config

import (
	"bufio"
	"fmt"
	"io"
	"os"

	"github.com/pelletier/go-toml/v2"
)

type database struct {
	Engine string
	Host   string
	Port   string
}

type server struct {
	Host string
	Port string
}

type config struct {
	Database database
	Server   server
}

func ReadConfig(path string) (config, error) {
	var cfg config

	configFile, err := os.Open(path)
	if err != nil {
		fmt.Println("Failed to read config file: ", err)
		return config{}, err
	}

	fileinfo, err := configFile.Stat()
	if err != nil {
		fmt.Println("Failed to get configfile info: ", err)
		return config{}, err
	}

	bs := make([]byte, fileinfo.Size())
	_, err = bufio.NewReader(configFile).Read(bs)
	if err != nil {
		fmt.Println("Failed to read config file: ", err)
		return config{}, err
	}

	err = toml.Unmarshal(bs, &cfg)
	if err != nil && err != io.EOF {
		fmt.Println("Failed to decode config file: ", err)
		return config{}, err
	}

	return cfg, nil
}
