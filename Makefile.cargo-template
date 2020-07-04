.PHONY: gitignore
gitignore: ## .gitignoreをDL
	curl --output .gitignore https://www.toptal.com/developers/gitignore/api/rust,intellij+all,linux,macos,windows,vim,emacs

.PHONY: build
build: ## cargo build
	cargo build

.PHONY: run
run: ## cargo run
	cargo run

.PHONY: check
check: ## cargo check
	cargo check

.PHONY: update
update: ## cargo update
	cargo update

.PHONY: doc
doc: ## cargo doc
	cargo doc

DOCKER_NGINX_TAG  := stable
DOCKER_NGINX_HOST := nginx
.PHONY: doc-docker
doc-docker: doc ## cargo docで作ったものをnginxを通して閲覧する
	( docker container ls | grep $(DOCKER_NGINX_HOST) ) \
	|| docker run \
		--detach \
		--rm \
		--interactive \
		--tty \
		--name $(DOCKER_NGINX_HOST) \
		--hostname $(DOCKER_NGINX_HOST) \
		--user $(id -u):$(id -g) \
		--publish 80:80 \
		--mount type=bind,source=$(PWD)/target/doc/,target=/usr/share/nginx/html/ \
		nginx:$(DOCKER_NGINX_TAG)
	echo Please open http://HOST/gussing_game/index.html

.PHONY: clean
clean: ## cargo doc-dockerでたてたコンテナを片付けする
	docker container ls --quiet --filter 'name=$(DOCKER_NGINX_HOST)' \
	| xargs -I {container-id} docker container stop {container-id}

.PHONY: fmt
fmt: ## cargo fmt
	cargo fmt

.PHONY: test
test: ## cargo test
	cargo test

.PHONY: help
help: ## Make タスク一覧
	@echo '######################################################################'
	@echo '# Makeタスク一覧'
	@echo '# $$ make XXX'
	@echo '# or'
	@echo '# $$ make XXX --dry-run'
	@echo '######################################################################'
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
	| grep --invert-match "## non-help" \
	| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
