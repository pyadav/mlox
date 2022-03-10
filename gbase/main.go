package gbase

import (
	"bufio"
	"errors"
	"fmt"
	"os"
)

func Run() {
	// Read program arguments and process
	args := os.Args[1:]

	if len(args) > 1 {
		fmt.Println("Usage: gox [script]")
		os.Exit(64)
	} else if len(args) == 1 {
		err := runFile(args[0])
		if err != nil {
			fmt.Printf("could not open file: %v \n", err)
			os.Exit(65)
		}
	} else {
		runPrompt()
	}
}

func runFile(path string) error {
	// process whole file content
	f, err := os.ReadFile(path)
	if err != nil {
		return errors.New("no file available")
	}

	run(string(f))
	return nil
}
func runPrompt() {
	inputScanner := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print("> ")
		if !inputScanner.Scan() {
			break
		}

		line := inputScanner.Text()
		run(line)
	}
}
func run(data string) {
	fmt.Println(data)
}
