# 📝 Todo CLI in Rust

A simple command-line todo list manager built with Rust.

### 🚀 Installation


   *  **Install Rust (if not installed)**:
      ```sh
         curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   * **Clone the repository**:
      ```sh
         git clone https://github.com/tomislav98/todo-cli.git
         cd todo_cli
   *  **Build the project**:
      ```sh
         cargo build --release
   * **Run the Setup command **:
      ```sh
         sudo chmod +x ./setup.sh
      ./setup.sh
   * **Allow Execution from Anywhere**
      ```sh
         sudo mv target/release/todo /usr/local/bin/todo
         Now you can just run:
         todo add "Walk the dog"
         todo
         todo done 1
         todo delete

# ✅ TODO List

## 📌 Pending Tasks
- [ ] Ui interface
- [ ] Sync with cloud
- [ ] Task 3: Description here

## 🎯 Completed Tasks
- [x] Add notification: Every time user inser the todo, youser would be notified.
- [x] Use Sqlite db: data are saved in db.
