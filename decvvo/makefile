COMPOSE_FILE=docker-compose.yml

.PHONY: all run stop

all: build run

run:
	docker-compose -f $(COMPOSE_FILE) up -d

stop:
	docker-compose -f $(COMPOSE_FILE) down

