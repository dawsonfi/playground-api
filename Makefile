integ-dev-aws:
	AWS_PROFILE=playground RUNNING_ENV=${USER} cargo test --verbose -F integration

integ-beta-aws:
	AWS_PROFILE=playground RUNNING_ENV=beta cargo test --verbose -F integration

test:
	cargo nextest run

test-log:
	cargo nextest run --nocapture

checks: test
	cargo clippy -- -D warnings && cargo fmt -- --check && cargo audit -d /tmp/tmp-advisory-db --ignore RUSTSEC-2020-0071

release: checks
	cargo lambda build --release --x86-64

cdk-install:
	npm --prefix dev/cdk install

cdk-build: cdk-install
	npm --prefix dev/cdk run build

bootstrap: release cdk-build
	cdk bootstrap --app "node dev/cdk/dist/index"

deploy-devo: release cdk-build
	cdk deploy $(USER)-playground-api-stack --profile playground --app "node dev/cdk/dist/index" --require-approval never

deploy-beta: release cdk-build
	cdk deploy beta-playground-api-stack --profile playground --app "node dev/cdk/dist/index" --require-approval never

run-local:
	AWS_PROFILE=playground cargo run | bunyan