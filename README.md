# Five Card Draw Poker

## 📌 Overview
This project is a **Five Card Draw Poker** game implemented in **Rust**, featuring a **WebSocket-based multiplayer system**. Players can join from different devices, participate in betting rounds, and play a complete game of poker in real-time.

## 🎮 Features
- 🃏 **Multiplayer Support:** Players can join lobbies via WebSockets.
- 💰 **Betting System:** Supports check, call, raise, fold, and all-in actions.
- 🔄 **Game Mechanics:** Standard 5-card draw rules, including drawing and replacing cards.
- 🎭 **Lobby Management:** Players can create, join, and list available lobbies.
- 🌐 **WebSocket Communication:** Real-time messaging for game updates.
- ⚡ **Asynchronous Execution:** Uses `tokio` for async networking.

## 🛠 Installation
### Prerequisites
- 📌 Rust (latest stable version)
- 📌 Cargo package manager

### Setup
Clone the repository:
```sh
git clone <repository_url>
cd five-card-draw-rust
```

Build the project:
```sh
cargo build --release
```

Run the server:
```sh
cargo run
```

## 🎲 How to Play
1. **Start the server** and connect via a WebSocket client.
2. **Join a lobby** or create a new one.
3. **Players place bets** and receive their initial 5 cards.
4. **Players can discard and replace up to 3 cards.**
5. **Final betting round** before the showdown.
6. **The winner is determined** based on standard poker hand rankings.

## 🔗 WebSocket Endpoints
| Endpoint           | Description |
|-------------------|-------------|
| `/lobby/create`   | Creates a new poker lobby |
| `/lobby/join`     | Joins an existing lobby |
| `/lobby/list`     | Lists available lobbies |
| `/game/bet`       | Places a bet |
| `/game/draw`      | Discards and draws new cards |
| `/game/showdown`  | Determines the winner |

## 📖 User Manual

### 7.1 Server Connection
1. Download **wscat**, a WebSocket client utility.
2. Install it using:
   ```sh
   npm install -g wscat
   ```
3. Run the server using:
   ```sh
   cargo run
   ```
4. In a new terminal, connect to the server:
   ```sh
   npx wscat -c ws://0.0.0.0:<port_address>/ws
   ```
5. If successful, you are now connected.

### 7.2 Registration Page
1. Once connected, you'll be prompted to **Sign In, Register, or Exit**.
2. If **Sign In** is chosen, enter your registered username.
3. If **Register** is chosen, enter a new username to create an account.
4. If **Exit** is chosen, you will disconnect from the server.

### 7.3 Lobby
1. In the lobby, the following options are available:
   - **Create Lobby:** `1 [lobby_name]` to create and join a new lobby.
   - **Join Lobby:** `2 [lobby_name]` to join an existing lobby.
   - **Show Recent Lobbies:** `3` to list active lobbies.
   - **View Stats:** `stats` to see player statistics.
   - **Quit:** `4` to disconnect.

### 7.4 Inside Lobby
1. The game starts when **at least 2 players** are in the lobby and ready.
2. Players can **ready up** by pressing `r`.
3. To **view players**, input `-p`.
4. To **view stats**, input `s`.
5. To **quit the lobby**, input `q`.
6. When all players are ready, the game begins.

### 7.5 In-Game
1. The game starts with the **ANTE round**, where all players place a minimum bet.
2. Players receive **5 cards**.
3. The first **betting round** starts. Options:
   - **Check**
   - **Raise** (input a valid amount)
   - **Call**
   - **Fold**
   - **All-In**
4. Next is the **Drawing round**, where players exchange cards:
   - **Stand Pat** (keep all cards)
   - **Exchange** `[indices to replace]`
5. A **second betting round** follows, same as the first.
6. **Showdown:** Players compare hands, and the winner is determined.

## 🚀 Technologies Used
- 🦀 **Rust**: Core language
- 🌍 **Warp**: Web framework for handling WebSockets
- ⚡ **Tokio**: Asynchronous runtime
- 🗃 **SQLx (if applicable)**: Database for storing player data

## 🔮 Future Enhancements
- 🤖 Add an **AI opponent** for solo play
- 🔑 Implement **player authentication**
- 📱 Add **mobile-friendly UI** using WebAssembly (Yew)

## 📜 License
This project is licensed under the **MIT License**.

---

**🚀 Contributions welcome!** If you’d like to improve the game, feel free to fork the repo and submit a PR.

