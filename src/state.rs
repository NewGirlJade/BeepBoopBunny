//global state which needs to persist as long as the game runs
struct GameState {
    game_settings: SettingsState,
    game_progress: u16,
}
//game settings data- pulled from save data if found, or sets defaults
struct SettingsState {
    //text_speed, words per minute, sound volume, enable flashing, tone picker (preset1..x or custom), pitch, noise, filtering, etc.
}
impl SettingsState {
    fn new() -> SettingsState {
        SettingsState {}
    }
    //should interact with the agb::save module to read from saves
    fn from_save() -> SettingsState {
        SettingsState {}
    }

    fn save_settings(&self) {}
}

enum TitleMenuState {
    MainMenu(MainMenuState),
    Settings(SettingsMenuState),
    InGame(InGameState),
}

struct MainMenuState {}
impl MainMenuState {
    fn update(&self) {}
    fn draw(&self) {}
}

struct SettingsMenuState {}
impl SettingsMenuState {
    fn update(&self) {}
    fn draw(&self) {}
}

struct InGameState {
    in_game_menu: InGameMenuState,
    location: LocationState,
}
impl InGameState {
    fn update(&self) {}
    fn draw(&self) {}
}

enum InGameMenuState {
    None,
    Settings(SettingsMenuState),
    InDictionary(InDictionary),
    InTapper(InTapper),
    InPuzzle(InPuzzle),
}

enum LocationState {
    RoomW,
    RoomN,
    RoomE,
    RoomS,
    Outside,
}

struct InDictionary {}

struct InTapper {
    framecounter: usize,
}

struct InPuzzle {}
