major:
	@VERSION=$$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	MAJOR=$$(echo $$VERSION | cut -d. -f1); \
	NEW_MAJOR=$$((MAJOR + 1)); \
	NEW_VERSION="$$NEW_MAJOR.0.0"; \
	sed -i "s/^version = \"$$VERSION\"/version = \"$$NEW_VERSION\"/" Cargo.toml; \
	echo "Bumped version $$VERSION -> $$NEW_VERSION"; \
	cargo test; \
	git add Cargo.toml; \
	git commit -m "Bump version to $$NEW_VERSION"; \
	git push origin main; \
	cargo publish

minor:
	@VERSION=$$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	MAJOR=$$(echo $$VERSION | cut -d. -f1); \
	MINOR=$$(echo $$VERSION | cut -d. -f2); \
	NEW_MINOR=$$((MINOR + 1)); \
	NEW_VERSION="$$MAJOR.$$NEW_MINOR.0"; \
	sed -i "s/^version = \"$$VERSION\"/version = \"$$NEW_VERSION\"/" Cargo.toml; \
	echo "Bumped version $$VERSION -> $$NEW_VERSION"; \
	cargo test; \
	git add Cargo.toml; \
	git commit -m "Bump version to $$NEW_VERSION"; \
	git push origin main; \
	cargo publish

patch:
	@VERSION=$$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	MAJOR=$$(echo $$VERSION | cut -d. -f1); \
	MINOR=$$(echo $$VERSION | cut -d. -f2); \
	PATCH=$$(echo $$VERSION | cut -d. -f3); \
	NEW_PATCH=$$((PATCH + 1)); \
	NEW_VERSION="$$MAJOR.$$MINOR.$$NEW_PATCH"; \
	sed -i "s/^version = \"$$VERSION\"/version = \"$$NEW_VERSION\"/" Cargo.toml; \
	echo "Bumped version $$VERSION -> $$NEW_VERSION"; \
	cargo test; \
	git add Cargo.toml; \
	git commit -m "Bump version to $$NEW_VERSION"; \
	git push origin main; \
	cargo publish
