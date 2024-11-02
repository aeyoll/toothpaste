.PHONY: all clean build-frontend build-backend package

# Configuration
TOOTHPASTE_API_URL ?= http://127.0.0.1:8080
DIST_DIR = dist
FRONTEND_DIR = crates/toothpaste-frontend
PACKAGE_NAME = toothpaste.zip

all: clean build-frontend build-backend build-cli package

clean:
	rm -rf $(DIST_DIR)

build-frontend:
	# Create dist directory
	mkdir -p $(DIST_DIR)
	# Build frontend with Trunk
	cd $(FRONTEND_DIR) && TOOTHPASTE_API_URL=$(TOOTHPASTE_API_URL) trunk build --release
	# Copy frontend build to dist
	cp -r $(FRONTEND_DIR)/dist/* $(DIST_DIR)/

build-cli:
	# Build cli in release mode
	cargo build --release -p toothpaste-cli
	# Copy cli binary to dist
	cp target/release/toothpaste-cli $(DIST_DIR)/

build-backend:
	# Build backend in release mode
	cargo build --release -p toothpaste-backend
	# Copy backend binary to dist
	cp target/release/toothpaste-backend $(DIST_DIR)/

package:
	zip -r $(PACKAGE_NAME) $(DIST_DIR)
