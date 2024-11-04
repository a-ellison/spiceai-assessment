.PHONY: build
build:
	docker build -t assessment-postgres-image postgres 
	docker build -t assessment-mysql-image mysql 
	cargo build

.PHONY: start
start:
	@echo "starting postgres container"
	@docker run --name assessment-postgres-container -p 5433:5432 -d assessment-postgres-image
	@echo "starting mysql container"
	@docker run --name assessment-mysql-container -p 5434:3306 -d assessment-mysql-image
	@echo "sleeping 10s to make sure databases are up"
	@sleep 10
	@echo "starting spice runtime"
	@bash start-spice.sh

.PHONY: run
run:
	@cargo run -q

.PHONY: stop
stop:
	@echo "stopping postgres container"
	-@docker rm -f assessment-postgres-container 2> /dev/null
	@echo "stopping mysql container"
	-@docker rm -f assessment-mysql-container 2> /dev/null
	@echo "stopping spice runtime"
	-@bash stop-spice.sh 2> /dev/null

.PHONY: clean
clean: stop
	-@docker image rm assessment-postgres-image 2> /dev/null
	-@docker image rm assessment-mysql-image 2> /dev/null
	-@docker image prune -a -f 2> /dev/null
