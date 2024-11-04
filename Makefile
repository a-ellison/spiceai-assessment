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
	@sleep 10 # make sure database servers are up
	@echo "starting spice runtime"
	@bash start-spice.sh

.PHONY: stop
stop:
	@echo "stopping postgres container"
	-@docker rm -f assessment-postgres-container 
	@echo "stopping mysql container"
	-@docker rm -f assessment-mysql-container 
	@echo "stopping spice runtime"
	-@bash stop-spice.sh

.PHONY: clean
clean:
	-docker rm -f assessment-postgres-container 
	-bash stop-spice.sh
	-docker image rm assessment-postgres-image
	-docker image prune -f


