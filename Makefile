all:
	docker compose up --build -d

logs:
	docker logs stockholm

stop:
	docker compose stop

clean: stop
	docker compose down

fclean: clean
	docker system prune -af

re: fclean all

test:
	docker exec -it stockholm /bin/bash

.Phony: all logs clean fclean