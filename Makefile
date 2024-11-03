.PHONY: build
build:
	docker build -t assessment-postgres-image postgres 
	cargo build

.PHONY: start
start:
	@echo "starting postgres container"
	@docker run --name assessment-postgres-container -p 5433:5432 -d assessment-postgres-image
	@echo "starting spice runtime"
	@bash start-spice.sh

.PHONY: stop
stop:
	@echo "stopping postgres container"
	-@docker rm -f assessment-postgres-container 
	@echo "stopping spice runtime"
	-@bash stop-spice.sh

.PHONY: clean
clean:
	-docker rm -f assessment-postgres-container 
	-bash stop-spice.sh
	-docker image rm assessment-postgres-image
	-docker image prune -f


