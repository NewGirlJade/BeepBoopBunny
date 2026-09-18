use agb::input::ButtonController;

//global state which needs to persist as long as the game runs
//pulled from save data if found, or sets defaults
struct GameState {
    input: agb::input::ButtonController,
    game_settings: SettingsState,
    game_progress: u16,
}
impl GameState {
    fn setup() -> GameState {
        //things which don't change based on save state
        let input = ButtonController::new();
        //if a save is detected

        /*placeholder code. I'll likely be asking agb's save/EEPROM API something like "does valid data exist here," which returns a bool or Option you'd match on — a good next rabbit hole*/
        let save = false;
        if save {
            Self::from_save(input)
        } else {
            Self::new(input)
        }
    }
    fn new(input: ButtonController) -> GameState {
        GameState {
            input: (input),
            game_settings: SettingsState::new(),
            game_progress: (0),
        }
    }
    fn from_save(input: ButtonController) -> GameState {
        GameState {
            input: (input),
            game_settings: (SettingsState::from_save()),
            game_progress: (0/*load progress from save file*/),
        }
    }
}

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
