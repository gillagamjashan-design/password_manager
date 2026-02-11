.PHONY: help install install-system clean run view all test lint format

# Default target - show help
.DEFAULT_GOAL := help

# Color output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m # No Color

# Project variables
PYTHON := python3
PIP := $(PYTHON) -m pip
VENV := venv
VENV_BIN := $(VENV)/bin
SCRIPT := soccer_roadmap_flowchart.py
OUTPUT := soccer_roadmap_flowchart.png

help: ## Show this help message
	@echo "$(BLUE)Soccer Roadmap Flowchart - Available Targets$(NC)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-20s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Examples:$(NC)"
	@echo "  make install    # Install dependencies"
	@echo "  make run        # Generate the flowchart"
	@echo "  make view       # View the generated flowchart"
	@echo "  make all        # Install, run, and view"

install: ## Install Python dependencies (creates virtual environment)
	@echo "$(BLUE)Creating virtual environment...$(NC)"
	@$(PYTHON) -m venv $(VENV)
	@echo "$(BLUE)Installing dependencies...$(NC)"
	@$(VENV_BIN)/pip install --upgrade pip
	@$(VENV_BIN)/pip install -r requirements.txt
	@echo "$(GREEN)✓ Dependencies installed successfully!$(NC)"
	@echo "$(YELLOW)Activate with: source $(VENV_BIN)/activate$(NC)"

install-system: ## Install dependencies system-wide (requires sudo for graphviz)
	@echo "$(BLUE)Installing system dependencies...$(NC)"
	@if command -v apt-get > /dev/null; then \
		echo "Detected Debian/Ubuntu system"; \
		sudo apt-get update && sudo apt-get install -y graphviz python3-pip; \
	elif command -v yum > /dev/null; then \
		echo "Detected RedHat/CentOS system"; \
		sudo yum install -y graphviz python3-pip; \
	elif command -v brew > /dev/null; then \
		echo "Detected macOS with Homebrew"; \
		brew install graphviz; \
	else \
		echo "$(RED)Could not detect package manager. Please install graphviz manually.$(NC)"; \
		exit 1; \
	fi
	@echo "$(BLUE)Installing Python packages...$(NC)"
	@$(PIP) install --user -r requirements.txt
	@echo "$(GREEN)✓ System dependencies installed successfully!$(NC)"

run: ## Generate the soccer roadmap flowchart
	@echo "$(BLUE)Generating soccer roadmap flowchart...$(NC)"
	@if [ -d "$(VENV)" ]; then \
		$(VENV_BIN)/python $(SCRIPT); \
	else \
		$(PYTHON) $(SCRIPT); \
	fi
	@echo "$(GREEN)✓ Flowchart generated!$(NC)"

view: ## View the generated flowchart (requires display)
	@if [ ! -f "$(OUTPUT)" ]; then \
		echo "$(RED)Error: $(OUTPUT) not found. Run 'make run' first.$(NC)"; \
		exit 1; \
	fi
	@echo "$(BLUE)Opening flowchart...$(NC)"
	@if command -v xdg-open > /dev/null; then \
		xdg-open $(OUTPUT); \
	elif command -v open > /dev/null; then \
		open $(OUTPUT); \
	elif command -v display > /dev/null; then \
		display $(OUTPUT); \
	else \
		echo "$(YELLOW)Could not find a viewer. Please open $(OUTPUT) manually.$(NC)"; \
	fi

all: install run view ## Install dependencies, generate flowchart, and view it

clean: ## Remove generated files and cache
	@echo "$(BLUE)Cleaning up...$(NC)"
	@rm -f $(OUTPUT)
	@rm -f soccer_roadmap_flowchart.dot
	@rm -rf __pycache__
	@rm -rf .pytest_cache
	@rm -rf *.pyc
	@echo "$(GREEN)✓ Cleanup complete!$(NC)"

clean-all: clean ## Remove generated files, cache, and virtual environment
	@echo "$(BLUE)Removing virtual environment...$(NC)"
	@rm -rf $(VENV)
	@echo "$(GREEN)✓ Full cleanup complete!$(NC)"

test: ## Test if dependencies are installed correctly
	@echo "$(BLUE)Testing dependencies...$(NC)"
	@if [ -d "$(VENV)" ]; then \
		$(VENV_BIN)/python -c "import graphviz; print('✓ graphviz OK')"; \
		$(VENV_BIN)/python -c "import matplotlib; print('✓ matplotlib OK')"; \
		$(VENV_BIN)/python -c "import networkx; print('✓ networkx OK')"; \
	else \
		$(PYTHON) -c "import graphviz; print('✓ graphviz OK')"; \
		$(PYTHON) -c "import matplotlib; print('✓ matplotlib OK')"; \
		$(PYTHON) -c "import networkx; print('✓ networkx OK')"; \
	fi
	@echo "$(GREEN)✓ All dependencies working!$(NC)"

lint: ## Check code style with flake8
	@echo "$(BLUE)Checking code style...$(NC)"
	@if [ -d "$(VENV)" ]; then \
		$(VENV_BIN)/python -m flake8 $(SCRIPT) --max-line-length=100 || true; \
	else \
		$(PYTHON) -m flake8 $(SCRIPT) --max-line-length=100 || true; \
	fi

format: ## Format code with black
	@echo "$(BLUE)Formatting code...$(NC)"
	@if [ -d "$(VENV)" ]; then \
		$(VENV_BIN)/python -m black $(SCRIPT) || echo "Install black with: pip install black"; \
	else \
		$(PYTHON) -m black $(SCRIPT) || echo "Install black with: pip install black"; \
	fi

info: ## Show project information
	@echo "$(BLUE)Soccer Roadmap Flowchart Generator$(NC)"
	@echo ""
	@echo "$(YELLOW)Project Structure:$(NC)"
	@ls -1 --color=never
	@echo ""
	@echo "$(YELLOW)Python Version:$(NC)"
	@$(PYTHON) --version
	@echo ""
	@if [ -d "$(VENV)" ]; then \
		echo "$(YELLOW)Virtual Environment:$(NC) $(GREEN)Active$(NC)"; \
	else \
		echo "$(YELLOW)Virtual Environment:$(NC) $(RED)Not created$(NC)"; \
	fi
	@echo ""
	@if [ -f "$(OUTPUT)" ]; then \
		echo "$(YELLOW)Generated Files:$(NC)"; \
		ls -lh $(OUTPUT) | awk '{print "  " $$9 " (" $$5 ")"}'; \
	else \
		echo "$(YELLOW)Generated Files:$(NC) $(RED)None (run 'make run' to generate)$(NC)"; \
	fi

dev-install: install ## Install development dependencies
	@echo "$(BLUE)Installing development dependencies...$(NC)"
	@$(VENV_BIN)/pip install black flake8 pytest
	@echo "$(GREEN)✓ Development dependencies installed!$(NC)"

quick: ## Quick run without installation check
	@$(PYTHON) $(SCRIPT)
