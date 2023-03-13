use tauri::{AppHandle, GlobalShortcutManager, Manager, Wry};

pub fn register_search_shortcut(app: AppHandle<Wry>) {
    let mut shortcut_manager = app.global_shortcut_manager();

    shortcut_manager
        .register("CommandOrControl+Space", move || {
            let search_bar_window = app
                .get_window("search-bar")
                .expect("the search bar window should be there");

            if search_bar_window.is_visible().unwrap() {
                search_bar_window.hide().unwrap();
            } else {
                search_bar_window.show().unwrap();
            }
        })
        .unwrap();
}
