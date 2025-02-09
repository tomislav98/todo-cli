# 📝 Todo CLI in Rust

A simple command-line todo list manager built with Rust.

### 🚀 Installation

- **Install Rust (if not installed)**:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  *You need also to install rclone comand to use sync comand.
  Set the .env file for rclone comand.
  example:
    LOCAL_PATH="Remote:/TODO/.todo_list.db"
    CLOUD_PATH="Remote:TODO"*
- **Clone the repository**:

  ```sh
  git clone https://github.com/tomislav98/todo-cli.git
  cd todo_cli
  ```

- **Run the Setup command**:
  ```sh
  sudo chmod +x ./setup.sh
  ./setup.sh
  ```
- **How to use**:
  ```sh
  todo add "Walk the dog" -d 2025-02-10
  todo -l
  todo done 1
  todo sync-cloud
  todo delete
  ```

# ✅ TODO List

## 📌 Pending Tasks

- [ ] Ui interface
- [x] Sync with cloud
- [ ] Task 3: Description here

## 🎯 Completed Tasks

- [x] Add notification: Every time user inser the todo, youser would be notified.
- [x] Use Sqlite db: data are saved in db.
- [x] Sync with cloud: added comand that sync data, done with rclone comand
