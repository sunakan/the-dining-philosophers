##################################################
# 変数(.gitignore)
##################################################
IGNORE_LIST    := rust,intellij+all,linux,macos,windows,vim,emacs
GIT_IGNORE_URL := https://www.toptal.com/developers/gitignore/api/$(IGNORE_LIST)

##################################################
# 変数/マクロ(Docker)
##################################################
DOCKER_RUST_TAG := 1-slim-buster
define docker-run-rust
  docker run \
    --rm \
    --interactive \
    --tty \
    --user `id -u`:`id -g` \
    --mount type=bind,source=$(PWD)/,target=$(PWD)/ \
    --workdir $(PWD) \
    rust:$(DOCKER_RUST_TAG) cargo
endef
CARGO_COMMAND := $(shell command -v cargo || echo $(docker-run-rust))

.PHONY: gitignore
gitignore: ## .gitignoreをDL
	curl --output .gitignore $(GIT_IGNORE_URL)

.PHONY: v
v: ## version確認
	@$(CARGO_COMMAND) --version

.PHONY: bash
bash: ## bash
	@$(CARGO_COMMAND) bash

.PHONY: build
build: ## cargo build
	@$(CARGO_COMMAND) build

.PHONY: run
run: ## cargo run
	@$(CARGO_COMMAND) run

.PHONY: check
check: ## cargo check
	@$(CARGO_COMMAND) check

.PHONY: doc
doc: ## cargo doc
	@$(CARGO_COMMAND) doc

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
		--user `id -u`:`id -g` \
		--publish 80:80 \
		--mount type=bind,source=$(PWD)/target/doc/,target=/usr/share/nginx/html/ \
		nginx:$(DOCKER_NGINX_TAG)
	echo Please open http://HOST/XXX/index.html

.PHONY: clean
clean: ## cargo コンテナやcargo buildの片付けする
	docker container ls --quiet --filter 'name=$(DOCKER_NGINX_HOST)' \
	| xargs -I {container-id} docker container stop {container-id}
	rm --recursive --force ./target

.PHONY: fmt
fmt: ## cargo fmt
	cargo fmt

.PHONY: test
test: ## cargo test
	@$(CARGO_COMMAND) test

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
