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
   * **Run the CLI**:
      ```sh
         ./target/release/todo add "Buy groceries"
         ./target/release/todo add "Finish Rust project"
         ./target/release/todo done "Buy groceries"
         ./target/release/todo delete
   * **Allow Execution from Anywhere**
      ```sh
         sudo mv target/release/todo /usr/local/bin/todo
         Now you can just run:
         todo add "Walk the dog"
         todo done "Walk the dog"
         todo delete
         You can add multiple todo at once:
         todo add "task1" "task2"
         To see the tasks just run:
         todo
