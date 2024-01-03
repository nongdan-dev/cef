fmt:
	rustfmt **/*.rs && \
	dprint fmt;

rebuild:
	bash scripts/rebuild.sh;

download:
	bash scripts/cef-download.sh;

update:
	cargo install-update -a && \
	cargo upgrade --incompatible && \
	dprint upgrade && \
	dprint config update;

git-re-init:
	rm -rf .git && \
	git init && \
	git add -A && \
	make -Bs fmt && \
	git add -A && \
	git commit -m "Initial commit" && \
	git remote add origin git@github.com:nongdan-dev/cellulose_fiber.git && \
	git push -uf origin master;

git-pull:
	git reset --hard && \
	git checkout -b tmp && \
	git branch -D master && \
	git fetch origin && \
	git checkout master && \
	git branch -D tmp;

r:
	clear && \
	make git-pull && \
	rm -rf target && \
	make rebuild && \
	cargo run --example raw_handler --features debug;
