
build:
	mkdir -p build
	go build -o build/ -ldflags "-s -w"
	
