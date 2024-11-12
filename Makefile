.PHONY: all clean build-frontend build-backend package

# Configuration
DIST_DIR = dist
FRONTEND_DIR = crates/toothpaste-frontend
VERSION ?= dev
TARGET ?= x86_64-unknown-linux-gnu
PACKAGE_NAME = toothpaste-$(VERSION)-$(TARGET).tar.gz

all: clean build-frontend build-backend build-cli package

clean:
	rm -rf $(DIST_DIR)

build-frontend:
	# Create dist directory
	mkdir -p $(DIST_DIR)
	# Build frontend with Trunk
	cd $(FRONTEND_DIR) && trunk build --release
	# Copy frontend build to dist
	cp -r $(FRONTEND_DIR)/dist/* $(DIST_DIR)/

build-cli:
	# Build cli in release mode
	cargo build --release -p toothpaste-cli --target $(TARGET)
	# Copy cli binary to dist
	cp target/$(TARGET)/release/toothpaste-cli $(DIST_DIR)/

build-backend:
	# Build backend in release mode
	cargo build --release -p toothpaste-backend --target $(TARGET)
	# Copy backend binary to dist
	cp target/$(TARGET)/release/toothpaste-backend $(DIST_DIR)/

package:
	tar -czf $(PACKAGE_NAME) -C $(DIST_DIR) .
