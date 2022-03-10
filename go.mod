module github.com/missingstack/intro

go 1.17

require (
	github.com/missingstack/gbase v1.0.0
)

replace (
	github.com/missingstack/gbase v1.0.0 => ./gbase
)
