# Soccer Player Development Roadmap Flowchart

A comprehensive Python-based flowchart visualizing the complete journey of a soccer player from beginner to professional and beyond.

## Overview

This project creates a detailed flowchart that maps out the key stages, decisions, and alternative pathways in a soccer player's career development. The visualization includes:

- **6 Main Stages**: Initial, Developmental, Intermediate, Advanced, Professional, and Post-Professional
- **Decision Points**: Continue training, injury recovery, career transitions
- **Alternative Pathways**: Recreational soccer, semi-professional routes, early coaching transitions
- **Career Options**: Coaching, management, media, business ventures, and lifelong involvement

## Features

- Color-coded stages for easy visual navigation
- Multiple decision points and alternative career paths
- Injury recovery pathways at different career stages
- Post-professional career options
- High-resolution output (300 DPI)
- Clean, professional visualization

## Prerequisites

### System Requirements

- Python 3.8 or higher
- Graphviz (system package)
- pip (Python package manager)

### Installing Graphviz

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install graphviz
```

**macOS:**
```bash
brew install graphviz
```

**Red Hat/CentOS:**
```bash
sudo yum install graphviz
```

## Quick Start

### Option 1: Using Make (Recommended)

```bash
# Show all available commands
make help

# Install dependencies and generate flowchart
make all

# Or step by step:
make install    # Install Python dependencies
make run        # Generate the flowchart
make view       # View the generated image
```

### Option 2: Manual Installation

```bash
# Create virtual environment
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Run the script
python soccer_roadmap_flowchart.py
```

## Makefile Targets

Run `make help` to see all available commands:

| Target | Description |
|--------|-------------|
| `help` | Show all available targets with descriptions |
| `install` | Create virtual environment and install dependencies |
| `install-system` | Install dependencies system-wide (requires sudo) |
| `run` | Generate the soccer roadmap flowchart |
| `view` | Open the generated flowchart image |
| `all` | Install, generate, and view in one command |
| `clean` | Remove generated files and cache |
| `clean-all` | Remove generated files, cache, and virtual environment |
| `test` | Test if all dependencies are installed correctly |
| `lint` | Check code style with flake8 |
| `format` | Format code with black |
| `info` | Show project information and status |
| `dev-install` | Install development dependencies |
| `quick` | Quick run without installation check |

## Output Files

After running the script, you'll find:

- `soccer_roadmap_flowchart.png` - The final flowchart image (300 DPI)
- `soccer_roadmap_flowchart.dot` - Source DOT file (for editing/reference)

## Flowchart Structure

### Stage Progression

1. **Initial Stage** (Light Blue)
   - Basic skills: dribbling, passing, shooting
   - Fitness foundation: agility, conditioning, coordination

2. **Developmental Stage** (Gold)
   - Youth teams and academies
   - Technical skills development
   - Tactical understanding

3. **Intermediate Stage** (Orange)
   - Competitive youth leagues
   - Advanced tactical training
   - Team strategy and match analysis

4. **Advanced Stage** (Red/Tomato)
   - Semi-professional leagues
   - Position specialization
   - Mental toughness and leadership

5. **Professional Stage** (Purple)
   - Professional club trials
   - Contract signing and management
   - Peak performance optimization

6. **Post-Professional Stage** (Plum)
   - Coaching and management
   - Media and business opportunities
   - Lifelong soccer involvement

### Decision Points

The flowchart includes critical decision points:

- **Continue Training?** - Choice between competitive and recreational paths
- **Elite Potential?** - Assessment for advancement to professional track
- **Injury Recovery** - Multiple pathways for rehabilitation and return
- **Professional Ready?** - Evaluation for professional career readiness
- **Career Transitions** - Options for early coaching or alternative careers

### Alternative Pathways

- Recreational soccer route
- Semi-professional career path
- Injury recovery and rehabilitation
- Transition to coaching/management
- Early retirement options

## Customization

To modify the flowchart, edit `soccer_roadmap_flowchart.py`:

- **Colors**: Adjust the `colors` dictionary
- **Content**: Modify node labels and descriptions
- **Structure**: Add/remove nodes and edges
- **Layout**: Change `rankdir` attribute ('TB' for top-to-bottom, 'LR' for left-to-right)
- **Size**: Adjust `size` and `dpi` attributes

Example:
```python
# Change to left-to-right layout
dot.attr(rankdir='LR', size='16,12', dpi='300')

# Add a new node
dot.node('new_node', 'New Stage', fillcolor='#FF0000')

# Add a connection
dot.edge('existing_node', 'new_node', label='Path')
```

## Dependencies

- **graphviz** (0.20.1): Flowchart generation and rendering
- **matplotlib** (3.8.2): Alternative visualization options
- **networkx** (3.2.1): Graph structure support

## Troubleshooting

### Graphviz Not Found

If you get an error about Graphviz not being found:
```bash
# Install the system package first
make install-system

# Then install Python dependencies
make install
```

### Permission Issues

If you encounter permission issues:
```bash
# Use virtual environment (recommended)
make install

# Or install with --user flag
pip install --user -r requirements.txt
```

### Display Issues

If `make view` doesn't work:
```bash
# Manually open the file
xdg-open soccer_roadmap_flowchart.png  # Linux
open soccer_roadmap_flowchart.png      # macOS
start soccer_roadmap_flowchart.png     # Windows
```

## Project Structure

```
soccer-flowchart/
├── soccer_roadmap_flowchart.py  # Main Python script
├── requirements.txt              # Python dependencies
├── Makefile                      # Build automation
├── README.md                     # This file
├── soccer_roadmap_flowchart.png  # Generated flowchart (after running)
└── soccer_roadmap_flowchart.dot  # DOT source file (after running)
```

## Examples

### Generate and View
```bash
make all
```

### Regenerate After Modifications
```bash
make clean
make run
```

### Check Installation
```bash
make test
```

### View Project Info
```bash
make info
```

## Contributing

To contribute to this project:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `make lint` and `make format`
5. Test with `make test`
6. Submit a pull request

## License

This project is open source and available for educational and personal use.

## Contact

For questions, issues, or suggestions, please open an issue in the project repository.

---

**Note**: This flowchart is a general guide and individual soccer career paths may vary based on location, opportunities, and personal circumstances.
