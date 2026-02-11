#!/bin/bash
set -e

echo "🚀 Password Manager - GitHub Upload Script"
echo "=========================================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if gh CLI is available
if command -v gh &> /dev/null; then
    echo -e "${BLUE}GitHub CLI detected!${NC}"
    echo ""
    echo "Choose your method:"
    echo "1) Use GitHub CLI (recommended - easiest)"
    echo "2) Use standard git (requires manual repo creation)"
    read -p "Enter choice (1 or 2): " choice

    if [ "$choice" == "1" ]; then
        echo ""
        echo -e "${YELLOW}Creating repository with GitHub CLI...${NC}"

        # Ask for repo visibility
        read -p "Make repository public? (y/n): " public

        if [ "$public" == "y" ] || [ "$public" == "Y" ]; then
            gh repo create password_manager --public --source=. --push \
                --description "🔐 Production-ready CLI password manager with AES-256-GCM encryption, Argon2id, TOTP, and advanced security features"
        else
            gh repo create password_manager --private --source=. --push \
                --description "🔐 Production-ready CLI password manager with AES-256-GCM encryption, Argon2id, TOTP, and advanced security features"
        fi

        echo ""
        echo -e "${GREEN}✅ Repository created and code pushed!${NC}"
        echo ""
        echo "View your repo at:"
        gh repo view --web
        exit 0
    fi
fi

# Standard git method
echo ""
echo -e "${YELLOW}Standard Git Upload${NC}"
echo ""
echo "First, create a new repository on GitHub:"
echo "  1. Go to: https://github.com/new"
echo "  2. Name: password_manager"
echo "  3. Description: 🔐 Production-ready CLI password manager"
echo "  4. Choose Public or Private"
echo "  5. DON'T initialize with README (we have one)"
echo "  6. Click 'Create repository'"
echo ""
read -p "Press Enter after creating the repository on GitHub..."

echo ""
read -p "Enter your GitHub username: " username

if [ -z "$username" ]; then
    echo -e "${YELLOW}Username required!${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}Setting up remote...${NC}"
git remote add origin "https://github.com/$username/password_manager.git"

echo -e "${BLUE}Pushing to GitHub...${NC}"
git push -u origin master || {
    echo ""
    echo -e "${YELLOW}If 'master' branch failed, trying 'main'...${NC}"
    git branch -M main
    git push -u origin main
}

echo ""
echo -e "${GREEN}✅ Successfully pushed to GitHub!${NC}"
echo ""
echo "View your repository at:"
echo "  https://github.com/$username/password_manager"
echo ""
echo -e "${GREEN}Next steps:${NC}"
echo "  1. Add topics: rust, password-manager, security, cli, encryption"
echo "  2. Enable Discussions (optional)"
echo "  3. Star your own repo!"
echo "  4. Share on Reddit's r/rust"
