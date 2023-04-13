
#!/usr/bin/env bash
set -e
echo "Buidling  decvvo..."
go build -tags netgo -ldflags '-s -w' -o  decvvo
echo "Exiting  decvvo..."