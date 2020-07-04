.PHONY: cargo-new-rust-01
cargo-new-rust-01: ## Rustプロジェクト01を作成(冪等)
	ls rust-01 \
	|| cargo new rust-01 --bin

.PHONY: distribute-makefile-for-rust
distribute-makefile-for-rust: ## Rust用のMakefileを配布(上書き)
	ls -1 ./*/Cargo.toml \
	| xargs -I {path} dirname {path} \
	| xargs -I {path} cp ./Makefile.cargo-template {path}/Makefile

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
