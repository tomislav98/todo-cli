use notify_rust::Notification;

pub fn send_notification(notification_type: &str) {
    match notification_type {
        "add" => send("You added item successfully."),
        "delete" => send("You delete item successfully."),
        "done" => send("You update the item succesfully."),
        _ => send("Unknown notification type."),
    }
}

fn send(msg: &str) {
    Notification::new()
        .summary("TODO List Updated")
        .body(msg)
        .show()
        .unwrap();
}
