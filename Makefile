CONTAINER_NAME="rome_naf"

build:
	docker-compose build

start:
	docker-compose down && docker-compose up -d

init:
	docker exec -it ${CONTAINER_NAME} bash
