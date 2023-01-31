integ-dev:
	cargo test --verbose -F integration

release:
	cargo lambda build --release --x86-64

cdk-install:
	npm --prefix dev/cdk install

cdk-build: cdk-install
	npm --prefix dev/cdk run build

bootstrap: release cdk-build
	cdk bootstrap --app "node dev/cdk/dist/index"

deploy-devo: release cdk-build
	cdk deploy $(USER)-playground-api-stack --profile playground --app "node dev/cdk/dist/index" --require-approval never