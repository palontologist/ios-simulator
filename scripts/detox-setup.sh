#!/usr/bin/env bash
# Detox E2E testing framework setup script
# Initializes Detox for automated iOS app testing

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
print_header() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ $1${NC}"
}

# Check prerequisites
print_header "Checking Prerequisites"

if ! command -v node &> /dev/null; then
    print_error "Node.js not found. Please run: nix develop .#detox"
    exit 1
fi
print_success "Node.js installed: $(node --version)"

if ! command -v npm &> /dev/null; then
    print_error "npm not found"
    exit 1
fi
print_success "npm installed: $(npm --version)"

# Install Detox
print_header "Installing Detox"

if [ ! -d "node_modules" ]; then
    print_info "Installing project dependencies..."
    npm install --save-dev detox-cli detox detox-server
else
    print_info "Dependencies already installed"
fi
print_success "Detox dependencies installed"

# Build Detox
print_header "Building Detox"

if [ -f "package.json" ]; then
    print_info "Found package.json - verifying Detox configuration"
    
    if ! jq '.detox' package.json &> /dev/null; then
        print_info "No Detox configuration found in package.json"
        print_info "Creating example Detox config..."
        
        cat > .detoxrc.json << 'EOF'
{
  "configurations": {
    "ios.sim.release": {
      "device": {
        "type": "iPhone 14"
      },
      "app": "ios.release"
    }
  },
  "apps": {
    "ios.release": {
      "type": "ios.app",
      "binaryPath": "ios/build/Build/Products/Release-iphonesimulator/YourApp.app",
      "build": "xcodebuild -workspace ios/YourApp.xcworkspace -scheme YourApp -configuration Release -sdk iphonesimulator -derivedDataPath ios/build"
    }
  },
  "testRunner": "jest"
}
EOF
        print_success "Example config created at .detoxrc.json"
    fi
else
    print_error "package.json not found in current directory"
    print_info "Please initialize a Node.js project first"
fi

# Build Detox
print_header "Building Native Dependencies"

if command -v detox &> /dev/null; then
    print_info "Building Detox for native testing..."
    detox build-framework-cache || print_info "Framework cache already built"
    print_success "Detox build dependencies ready"
else
    print_info "Install Detox CLI with: npm install -g detox-cli"
fi

# Print usage instructions
print_header "Detox Setup Complete"

echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo ""
echo "1. Configure your app in .detoxrc.json (if not already done)"
echo ""
echo "2. Start the simulator connection:"
echo "   export MAC_HOST='your-mac.local'"
echo "   export MAC_USER='username'"
echo "   ./scripts/connect-simulator.sh"
echo ""
echo "3. Run your first test:"
echo "   detox build-framework-cache"
echo "   detox build e2e --configuration ios.sim.release"
echo "   detox test e2e --configuration ios.sim.release --cleanup"
echo ""
echo "4. Create E2E tests in: e2e/ directory"
echo ""
echo -e "${YELLOW}Useful commands:${NC}"
echo "   detox --help"
echo "   detox test --help"
echo "   detox doctor"
echo ""
