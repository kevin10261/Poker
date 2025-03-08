# Five Card Draw Poker (Rust + WebSocket)

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

