# Soccer Game - Gameplay Guide

## 🎮 How to Run the Game

```bash
make run
```

Or:
```bash
./soccer_game
```

---

## 📋 Interactive Menu

When you start the game, you'll see an interactive menu:

```
========================================
        SOCCER GAME - MAIN MENU
========================================

Select Game Mode:
  [1] 1v1 - One player per team
  [2] 2v2 - Two players per team
  [3] 3v3 - Three players per team

Enter your choice (1-3):
```

**Simply type 1, 2, or 3 and press Enter!**

---

## 🕹️ Controls

| Key | Action |
|-----|--------|
| **Arrow Keys** | Move your player (Blue team, leftmost player) |
| **Space** | Kick/shoot the ball |
| **ESC** | Exit the game |

---

## 🎯 Game Features

### 1. **Player Boundaries**
- ✅ Players **cannot move outside the pitch**
- When you try to move past the edge, your player stops at the boundary
- This applies to both human and AI players

### 2. **Ball Out-of-Bounds System**
- ✅ When the ball goes **out of bounds**, the game:
  - **Pauses briefly**
  - Shows a **yellow notification**: "OUT OF BOUNDS! [TEAM] gets possession"
  - **Resets the ball** near the boundary
  - Gives possession to the **team that didn't kick it out**

**Example:**
- Blue team kicks the ball out on the left side
- Red team gets possession
- A red player moves to the ball's new position

### 3. **Multiple Game Modes**

#### 1v1 Mode
- 1 Blue player (you control)
- 1 Red player (AI)
- Fast-paced, easier to score

#### 2v2 Mode
- 2 Blue players (you control the left one)
- 2 Red players (both AI)
- Balanced gameplay

#### 3v3 Mode
- 3 Blue players (you control the left one)
- 3 Red players (all AI)
- Classic soccer feel, more strategic

---

## 📊 Game Info Display

**On-screen displays:**
- **Top center**: Score (Blue X - X Red)
- **Below score**: Match timer (MM:SS format)
- **Bottom left**: Current game mode (1v1, 2v2, or 3v3)
- **Center (when triggered)**: Out-of-bounds notifications

---

## 🏆 Winning the Game

- Match duration: **3 minutes** (180 seconds)
- Score by kicking the ball into the opponent's goal
- **Blue goal**: Left side (Red team scores here)
- **Red goal**: Right side (Blue team scores here)
- At the end, the team with more goals wins!

---

## 🧪 Testing the Features

### Test Player Boundaries:
1. Start the game
2. Move your player (arrow keys) to the edges
3. **You should see:** Player stops at the edge, can't go off-screen

### Test Out-of-Bounds:
1. Start the game
2. Kick the ball (Space) towards the **top or bottom edge**
3. **You should see:**
   - Yellow notification appears: "OUT OF BOUNDS! [TEAM] gets possession"
   - Ball resets near the boundary
   - Console message: "Out of bounds! Team X gets possession"
   - Opposing team's player moves to the ball

### Test Different Modes:
1. Run `./soccer_game`
2. Select mode **1** (1v1)
3. Play a bit, then restart
4. Select mode **2** (2v2) - notice 2 players per team
5. Select mode **3** (3v3) - full teams

---

## 🛠️ Rebuilding After Changes

If you modify the code:

```bash
make clean   # Remove old build
make         # Rebuild
make run     # Run the game
```

Or all at once:
```bash
make clean && make run
```

---

## 🐛 Troubleshooting

**Issue: Menu doesn't appear**
- Make sure you rebuilt: `make clean && make`

**Issue: Out-of-bounds doesn't work**
- The ball must go **completely outside** the pitch boundaries
- It won't trigger in the goal areas (left/right center)

**Issue: Can't see notifications**
- Make sure fonts loaded (check console for font warnings)
- Notifications appear for 2 seconds in the center of the screen

---

## 📝 Summary of New Features

✅ **Interactive menu** - Select 1v1, 2v2, or 3v3 before playing
✅ **Player boundaries** - Players can't leave the pitch
✅ **Out-of-bounds detection** - Ball going out gives possession to other team
✅ **Visual notifications** - Yellow on-screen messages when ball goes out
✅ **Game mode display** - See current mode in bottom-left corner

**Enjoy the game!** ⚽
